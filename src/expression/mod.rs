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
    pub fn evaluate(self) -> Self {
        let mut current = self;
        while let Expression::Application(application) = current {
            current = application.evaluate1();
            println!("{}\n", current);
        }
        current
    }

    fn shifted(self, d: isize, c: usize) -> Self {
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

    fn substituted(self, j: usize, term: Expression) -> Self {
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

    pub fn from_ast<'a>(
        value: &'a ast::Expression,
        table: &mut HashMap<&'a str, usize>,
    ) -> Expression {
        match value {
            ast::Expression::Variable(variable) => {
                Expression::Variable(Variable::from_ast(variable, table))
            }

            ast::Expression::Abstraction(abstraction) => {
                Expression::Abstraction(Abstraction::from_ast(abstraction, table))
            }

            ast::Expression::Application(application) => Application::from_ast(application, table), // FIXME: looking weird. the associated function `Application::from_ast` returns `Expression`.
        }
    }

    pub fn from_ast_program<'a>(
        value: &'a ast::Program,
        table: &mut HashMap<&'a str, usize>,
    ) -> Expression {
        let ast::Program(statements) = value;

        let mut iter = statements.iter().rev();
        if let Some(ast::Statement::Expression(ast::ExpressionStatement { expression: result })) =
            iter.next()
        {
            iter.fold(
                Expression::from_ast(result, table),
                |result, statement| match statement {
                    ast::Statement::Expression(..) => unimplemented!(),

                    ast::Statement::Let(ast::LetStatement {
                        variable: ast::Identifier(variable),
                        expression,
                    }) => {
                        let outer = table.get(variable.as_str()).cloned();
                        table.iter_mut().for_each(|(_, i)| *i += 1);
                        table.insert(variable.as_str(), 0);

                        let result = Expression::Application(Application::new(
                            Expression::Abstraction(Abstraction::new(variable.to_owned(), result)),
                            Expression::from_ast(expression, table),
                        ));

                        table.remove(variable.as_str());
                        table.iter_mut().for_each(|(_, i)| *i -= 1);
                        if let Some(i) = outer {
                            table.insert(variable.as_str(), i);
                        }

                        result
                    }
                },
            )
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
        let result = Expression::from_ast(
            &ast::Expression::from(ast::AbstractionExpression::new(
                vec![ast::Identifier::new("x"), ast::Identifier::new("x")],
                ast::VariableExpression::new(ast::Identifier::new("x")),
            )),
            &mut HashMap::new(),
        );

        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Abstraction(Abstraction::new(
                "x",
                Expression::Variable(Variable::new(Some(0), "x")),
            )),
        ));
        assert_eq!(expected, result);

        let b = Expression::from_ast(
            &ast::Expression::from(ast::AbstractionExpression::new(
                vec![ast::Identifier::new("x")],
                ast::ApplicationExpression::new(vec![
                    ast::Expression::from(ast::AbstractionExpression::new(
                        vec![ast::Identifier::new("x")],
                        ast::VariableExpression::new(ast::Identifier::new("x")),
                    )),
                    ast::Expression::from(ast::VariableExpression::new(ast::Identifier::new("x"))),
                ]),
            )),
            &mut HashMap::new(),
        );
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
        let a = Expression::from_ast(
            &ast::Expression::from(ast::ApplicationExpression::new(vec![
                ast::Expression::from(ast::VariableExpression::new(ast::Identifier::new("a"))),
                ast::Expression::from(ast::VariableExpression::new(ast::Identifier::new("b"))),
                ast::Expression::from(ast::VariableExpression::new(ast::Identifier::new("c"))),
            ])),
            &mut HashMap::new(),
        );
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
        let result = Expression::from_ast_program(
            &ast::Program(vec![
                ast::Statement::from(ast::LetStatement::new(
                    ast::Identifier::new("id"),
                    ast::Expression::from(ast::AbstractionExpression::new(
                        vec![ast::Identifier::new("x")],
                        ast::Expression::from(ast::ApplicationExpression::new(vec![
                            ast::Expression::from(ast::VariableExpression::new(
                                ast::Identifier::new("x"),
                            )),
                        ])),
                    )),
                )),
                ast::Statement::from(ast::ExpressionStatement::new(ast::Expression::from(
                    ast::VariableExpression::new(ast::Identifier::new("id")),
                ))),
            ]),
            &mut HashMap::new(),
        );
        println!("expected: {}", expected);
        println!("result  : {}", result);
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
