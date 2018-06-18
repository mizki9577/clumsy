mod abstraction;
mod application;
mod variable;
pub use self::abstraction::*;
pub use self::application::*;
pub use self::variable::*;

use ast;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Variable(Variable),
    Abstraction(Abstraction),
    Application(Application),
}

impl Expression {
    pub fn assign_indices<'a>(&'a mut self, table: &mut HashMap<&'a str, usize>) {
        match self {
            Expression::Variable(variable) => variable.assign_indices(table),
            Expression::Abstraction(abstraction) => abstraction.assign_indices(table),
            Expression::Application(application) => application.assign_indices(table),
        }
    }

    pub fn evaluate(self) -> Expression {
        let mut current = self;
        while let Expression::Application(application) = current {
            current = application.evaluate1();
        }
        current
    }

    fn shifted(self, d: isize, c: usize) -> Expression {
        match self {
            Expression::Variable(variable) => Expression::Variable(variable.shifted(d, c)),
            Expression::Abstraction(abstraction) => {
                Expression::Abstraction(abstraction.shifted(d, c))
            }
            Expression::Application(application) => {
                Expression::Application(application.shifted(d, c))
            }
        }
    }

    fn substituted(self, j: usize, term: Expression) -> Expression {
        match self {
            Expression::Variable(variable) => variable.substituted(j, term),
            Expression::Abstraction(abstraction) => {
                Expression::Abstraction(abstraction.substituted(j, term))
            }
            Expression::Application(application) => {
                Expression::Application(application.substituted(j, term))
            }
        }
    }
}

impl<'a> From<&'a ast::Expression> for Expression {
    fn from(value: &ast::Expression) -> Expression {
        let mut result = match value {
            ast::Expression::Variable(ast::VariableExpression { identifier }) => {
                Expression::Variable(Variable::from(identifier))
            }

            ast::Expression::Abstraction(abstraction) => {
                Expression::Abstraction(Abstraction::from(abstraction))
            }

            ast::Expression::Application(application) => Expression::from(application),
        };

        result.assign_indices(&mut HashMap::new());
        result
    }
}

impl<'a> From<&'a ast::Program> for Expression {
    fn from(value: &ast::Program) -> Expression {
        let ast::Program(statements) = value;

        let mut iter = statements.iter();
        if let Some(ast::Statement::Expression(ast::ExpressionStatement { expression: result })) =
            iter.next_back()
        {
            let mut result = iter.rfold(Expression::from(result), |result, statement| {
                match statement {
                    ast::Statement::Expression(..) => unimplemented!(),
                    ast::Statement::Let(ast::LetStatement {
                        variable: ast::Identifier(variable),
                        expression,
                    }) => Expression::Application(Application::new(
                        Expression::Abstraction(Abstraction::new(variable.to_owned(), result)),
                        expression,
                    )),
                }
            });
            result.assign_indices(&mut HashMap::new());
            result
        } else {
            unimplemented!()
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expression::Variable(variable) => variable.fmt(f),
            Expression::Abstraction(abstraction) => abstraction.fmt(f),
            Expression::Application(application) => application.fmt(f),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn translate_abstraction() {
        let result = Expression::from(&ast::Expression::from(ast::AbstractionExpression::new(
            vec![ast::Identifier::new("x"), ast::Identifier::new("x")],
            ast::VariableExpression::new(ast::Identifier::new("x")),
        )));

        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Abstraction(Abstraction::new(
                "x",
                Expression::Variable(Variable::new(Some(0), "x")),
            )),
        ));
        assert_eq!(expected, result);

        let b = Expression::from(&ast::Expression::from(ast::AbstractionExpression::new(
            vec![ast::Identifier::new("x")],
            ast::ApplicationExpression::new(vec![
                ast::Expression::from(ast::AbstractionExpression::new(
                    vec![ast::Identifier::new("x")],
                    ast::VariableExpression::new(ast::Identifier::new("x")),
                )),
                ast::Expression::from(ast::VariableExpression::new(ast::Identifier::new("x"))),
            ]),
        )));
        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Application(Application::new(
                Expression::Abstraction(Abstraction::new(
                    "x",
                    Expression::Variable(Variable::new(Some(0), "x")),
                )),
                Expression::Variable(Variable::new(Some(0), "x")),
            )),
        ));
        assert_eq!(expected, b);
    }

    #[test]
    fn translate_application() {
        let a = Expression::from(&ast::Expression::from(ast::ApplicationExpression::new(
            vec![
                ast::Expression::from(ast::VariableExpression::new(ast::Identifier::new("a"))),
                ast::Expression::from(ast::VariableExpression::new(ast::Identifier::new("b"))),
                ast::Expression::from(ast::VariableExpression::new(ast::Identifier::new("c"))),
            ],
        )));
        let expected = Expression::Application(Application::new(
            Expression::Application(Application::new(
                Expression::Variable(Variable::new(None, "a")),
                Expression::Variable(Variable::new(None, "b")),
            )),
            Expression::Variable(Variable::new(None, "c")),
        ));
        assert_eq!(expected, a);
    }

    #[test]
    fn translate_let_statement() {
        let expected = Expression::Application(Application::new(
            Expression::Abstraction(Abstraction::new(
                "id",
                Expression::Variable(Variable::new(0, "id")),
            )),
            Expression::Abstraction(Abstraction::new(
                "x",
                Expression::Variable(Variable::new(0, "x")),
            )),
        ));
        let result = Expression::from(&ast::Program(vec![
            ast::Statement::from(ast::LetStatement::new(
                ast::Identifier::new("id"),
                ast::Expression::from(ast::AbstractionExpression::new(
                    vec![ast::Identifier::new("x")],
                    ast::Expression::from(ast::ApplicationExpression::new(vec![
                        ast::Expression::from(ast::VariableExpression::new(ast::Identifier::new(
                            "x",
                        ))),
                    ])),
                )),
            )),
            ast::Statement::from(ast::ExpressionStatement::new(ast::Expression::from(
                ast::VariableExpression::new(ast::Identifier::new("id")),
            ))),
        ]));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_shift() {
        let expected = Expression::Variable(Variable::new(1, "x"));
        let result = Expression::Variable(Variable::new(0, "x")).shifted(1, 0);
        assert_eq!(expected, result);

        let expected = Expression::Variable(Variable::new(0, "x"));
        let result = Expression::Variable(Variable::new(0, "x")).shifted(1, 1);
        assert_eq!(expected, result);

        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(2, "y")),
        ));
        let result = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(1, "y")),
        )).shifted(1, 0);
        assert_eq!(expected, result);

        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(0, "x")),
        ));
        let result = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(0, "x")),
        )).shifted(1, 0);
        assert_eq!(expected, result);

        let expected = Expression::Application(Application::new(
            Expression::Variable(Variable::new(1, "x")),
            Expression::Variable(Variable::new(2, "y")),
        ));
        let result = Expression::Application(Application::new(
            Expression::Variable(Variable::new(0, "x")),
            Expression::Variable(Variable::new(1, "y")),
        )).shifted(1, 0);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_substitute() {
        let expected = Expression::Variable(Variable::new(None, "a"));
        let result = Expression::Variable(Variable::new(0, "x"))
            .substituted(0, Expression::Variable(Variable::new(None, "a")));
        assert_eq!(expected, result);

        let expected = Expression::Variable(Variable::new(1, "x"));
        let result = Expression::Variable(Variable::new(1, "x"))
            .substituted(0, Expression::Variable(Variable::new(None, "a")));
        assert_eq!(expected, result);

        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(None, "a")),
        ));
        let result = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(1, "y")),
        )).substituted(0, Expression::Variable(Variable::new(None, "a")));
        assert_eq!(expected, result);

        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(0, "x")),
        ));
        let result = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(0, "x")),
        )).substituted(0, Expression::Variable(Variable::new(None, "a")));
        assert_eq!(expected, result);

        let expected = Expression::Application(Application::new(
            Expression::Variable(Variable::new(0, "x")),
            Expression::Variable(Variable::new(None, "a")),
        ));
        let result = Expression::Application(Application::new(
            Expression::Variable(Variable::new(0, "x")),
            Expression::Variable(Variable::new(1, "y")),
        )).substituted(1, Expression::Variable(Variable::new(None, "a")));
        assert_eq!(expected, result);
    }
}