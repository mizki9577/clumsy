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
    fn assign_indices(&mut self) {
        fn assign_indices_impl<'a>(value: &'a mut Expression, table: &mut HashMap<&'a str, usize>) {
            match value {
                Expression::Variable(Variable { name, index }) => {
                    *index = table.get(name.as_str()).cloned();
                }

                Expression::Abstraction(Abstraction { name, expression }) => {
                    let outer = table.get(name.as_str()).cloned();
                    table.iter_mut().for_each(|(_, i)| *i += 1);
                    table.insert(name, 0);

                    assign_indices_impl(expression, table);

                    table.remove(name.as_str());
                    table.iter_mut().for_each(|(_, i)| *i -= 1);
                    if let Some(i) = outer {
                        table.insert(name, i);
                    }
                }

                Expression::Application(Application { callee, argument }) => {
                    assign_indices_impl(callee, table);
                    assign_indices_impl(argument, table);
                }
            }
        }

        assign_indices_impl(self, &mut HashMap::new())
    }

    pub fn evaluate(self) -> Self {
        fn evaluate1(value: Expression) -> Result<Expression, Expression> {
            match value {
                Expression::Application(Application {
                    callee: box Expression::Abstraction(Abstraction { expression, .. }),
                    box argument,
                }) => Ok(expression
                    .substituted(0, argument.shifted(1, 0))
                    .shifted(-1, 0)),

                Expression::Application(Application {
                    callee: box callee,
                    box argument,
                }) => match evaluate1(callee) {
                    Ok(callee) => Ok(Expression::Application(Application {
                        callee: box callee,
                        argument: box argument,
                    })),
                    Err(callee) => Err(Expression::Application(Application {
                        callee: box callee,
                        argument: box argument,
                    })),
                },

                _ => Err(value),
            }
        }

        match evaluate1(self) {
            Ok(result) => result.evaluate(),
            Err(result) => result,
        }
    }

    fn shifted(self, d: isize, c: usize) -> Self {
        match self {
            Expression::Variable(Variable {
                index: Some(index),
                ref name,
            }) if index >= c =>
            {
                Expression::Variable(Variable {
                    index: Some((index as isize + d) as usize),
                    name: name.to_owned(),
                })
            }

            Expression::Variable(_) => self,

            Expression::Abstraction(Abstraction { name, expression }) => {
                Expression::Abstraction(Abstraction {
                    name,
                    expression: box expression.shifted(d, c + 1),
                })
            }

            Expression::Application(Application { callee, argument }) => {
                Expression::Application(Application {
                    callee: box callee.shifted(d, c),
                    argument: box argument.shifted(d, c),
                })
            }
        }
    }

    fn substituted(self, j: usize, term: Expression) -> Self {
        match self {
            Expression::Variable(Variable {
                index: Some(index), ..
            }) if index == j =>
            {
                term
            }

            Expression::Variable(_) => self,

            Expression::Abstraction(Abstraction { name, expression }) => {
                Expression::Abstraction(Abstraction {
                    name,
                    expression: box expression.substituted(j + 1, term.shifted(1, 0)),
                })
            }

            Expression::Application(Application { callee, argument }) => {
                let cloned_term = term.clone();
                Expression::Application(Application {
                    callee: box callee.substituted(j, term),
                    argument: box argument.substituted(j, cloned_term),
                })
            }
        }
    }
}

impl<'a> From<&'a ast::Expression> for Expression {
    fn from(value: &ast::Expression) -> Self {
        let mut result = match value {
            ast::Expression::Variable(ast::VariableExpression { identifier }) => {
                Expression::Variable(identifier.into())
            }

            ast::Expression::Abstraction(abstraction) => {
                Expression::Abstraction(abstraction.into())
            }

            ast::Expression::Application(application) => application.into(),
        };

        result.assign_indices();
        result
    }
}

impl<'a> From<&'a ast::Program> for Expression {
    fn from(value: &ast::Program) -> Self {
        let ast::Program(statements) = value;

        let mut iter = statements.iter().rev();
        if let Some(ast::Statement::Expression(ast::ExpressionStatement { expression: result })) =
            iter.next()
        {
            let mut result = iter.fold(result.into(), |result, statement| match statement {
                ast::Statement::Expression(..) => unimplemented!(),
                ast::Statement::Let(ast::LetStatement {
                    variable: ast::Identifier(variable),
                    expression,
                }) => Expression::Application(Application {
                    callee: box Expression::Abstraction(Abstraction {
                        name: variable.to_owned(),
                        expression: box result,
                    }),
                    argument: box expression.into(),
                }),
            });
            result.assign_indices(); // FIXME: We are currently calling this twice. DAS IST GUT NICHT.
            result
        } else {
            unimplemented!()
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expression::Abstraction(Abstraction { name, expression }) => {
                write!(f, r"(\{}. {})", name, expression)
            }
            Expression::Application(Application { callee, argument }) => {
                write!(f, r"({} {})", callee, argument)
            }
            Expression::Variable(Variable { name, .. }) => write!(f, r"{}", name),
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
    fn test_shift() {
        let expected = Expression::Variable(Variable::new(Some(1), "x"));
        let result = Expression::Variable(Variable::new(Some(0), "x")).shifted(1, 0);
        assert_eq!(expected, result);

        let expected = Expression::Variable(Variable::new(Some(0), "x"));
        let result = Expression::Variable(Variable::new(Some(0), "x")).shifted(1, 1);
        assert_eq!(expected, result);

        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(2), "y")),
        ));
        let result = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(1), "y")),
        )).shifted(1, 0);
        assert_eq!(expected, result);

        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(0), "x")),
        ));
        let result = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(0), "x")),
        )).shifted(1, 0);
        assert_eq!(expected, result);

        let expected = Expression::Application(Application::new(
            Expression::Variable(Variable::new(Some(1), "x")),
            Expression::Variable(Variable::new(Some(2), "y")),
        ));
        let result = Expression::Application(Application::new(
            Expression::Variable(Variable::new(Some(0), "x")),
            Expression::Variable(Variable::new(Some(1), "y")),
        )).shifted(1, 0);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_substitute() {
        let expected = Expression::Variable(Variable::new(None, "a"));
        let result = Expression::Variable(Variable::new(Some(0), "x"))
            .substituted(0, Expression::Variable(Variable::new(None, "a")));
        assert_eq!(expected, result);

        let expected = Expression::Variable(Variable::new(Some(1), "x"));
        let result = Expression::Variable(Variable::new(Some(1), "x"))
            .substituted(0, Expression::Variable(Variable::new(None, "a")));
        assert_eq!(expected, result);

        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(None, "a")),
        ));
        let result = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(1), "y")),
        )).substituted(0, Expression::Variable(Variable::new(None, "a")));
        assert_eq!(expected, result);

        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(0), "x")),
        ));
        let result = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(0), "x")),
        )).substituted(0, Expression::Variable(Variable::new(None, "a")));
        assert_eq!(expected, result);

        let expected = Expression::Application(Application::new(
            Expression::Variable(Variable::new(Some(0), "x")),
            Expression::Variable(Variable::new(None, "a")),
        ));
        let result = Expression::Application(Application::new(
            Expression::Variable(Variable::new(Some(0), "x")),
            Expression::Variable(Variable::new(Some(1), "y")),
        )).substituted(1, Expression::Variable(Variable::new(None, "a")));
        assert_eq!(expected, result);
    }
}
