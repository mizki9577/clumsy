use interpreter::{Abstraction, Application, Variable};

use parser::ast::{ASTAbstraction, ASTApplication, ASTIdentifier, AST};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Abstraction(Abstraction),
    Application(Application),
    Variable(Variable),
}

impl Expression {
    // Is this should be immutable?
    fn assign_indices(&mut self) {
        self.assign_indices_impl(&mut HashMap::new())
    }

    fn assign_indices_impl<'a>(&'a mut self, table: &mut HashMap<&'a str, usize>) {
        match self {
            Expression::Abstraction(Abstraction { name, expression }) => {
                let outer = table.get(name.as_str()).cloned();
                table.iter_mut().for_each(|(_, i)| *i += 1);
                table.insert(name, 0);

                expression.assign_indices_impl(table);

                table.remove(name.as_str());
                table.iter_mut().for_each(|(_, i)| *i -= 1);
                if let Some(i) = outer {
                    table.insert(name, i);
                }
            }
            Expression::Application(Application { callee, argument }) => {
                callee.assign_indices_impl(table);
                argument.assign_indices_impl(table);
            }
            Expression::Variable(Variable { name, index }) => {
                *index = table.get(name.as_str()).cloned();
            }
        }
    }

    pub fn evaluate(self) -> Self {
        match self.evaluate1() {
            Ok(result) => result.evaluate(),
            Err(result) => result,
        }
    }

    fn evaluate1(self) -> Result<Self, Self> {
        match self {
            Expression::Application(Application {
                callee: box Expression::Abstraction(Abstraction { expression, .. }),
                box argument,
            }) => Ok(expression.substituted(0, argument.shifted(0)).unshifted(0)),

            Expression::Application(Application {
                callee: box Expression::Application(callee),
                box argument,
            }) => match Expression::Application(callee).evaluate1() {
                Ok(callee) => Ok(Expression::Application(Application::new(callee, argument))),
                Err(callee) => Err(Expression::Application(Application::new(callee, argument))),
            },

            _ => Err(self),
        }
    }

    fn shifted(self, c: usize) -> Self {
        self.shift_impl(true, c)
    }

    fn unshifted(self, c: usize) -> Self {
        self.shift_impl(false, c)
    }

    fn shift_impl(self, increment: bool, c: usize) -> Self {
        match self {
            Expression::Abstraction(Abstraction { name, expression }) => {
                Expression::Abstraction(Abstraction {
                    name,
                    expression: box expression.shift_impl(increment, c + 1),
                })
            }
            Expression::Application(Application { callee, argument }) => {
                Expression::Application(Application {
                    callee: box callee.shift_impl(increment, c),
                    argument: box argument.shift_impl(increment, c),
                })
            }
            Expression::Variable(Variable {
                index: Some(index),
                ref name,
            }) if index >= c =>
            {
                Expression::Variable(Variable {
                    index: Some(if increment { index + 1 } else { index - 1 }),
                    name: name.to_owned(),
                })
            }
            _ => self,
        }
    }

    fn substituted(self, j: usize, term: Expression) -> Self {
        match self {
            Expression::Abstraction(Abstraction { name, expression }) => {
                Expression::Abstraction(Abstraction {
                    name,
                    expression: box expression.substituted(j + 1, term.shifted(0)),
                })
            }
            Expression::Application(Application { callee, argument }) => {
                let cloned_term = term.clone();
                Expression::Application(Application {
                    callee: box callee.substituted(j, term),
                    argument: box argument.substituted(j, cloned_term),
                })
            }
            Expression::Variable(Variable {
                index: Some(index), ..
            }) if index == j =>
            {
                term
            }
            _ => self,
        }
    }
}

impl<'a> From<&'a AST> for Expression {
    fn from(value: &AST) -> Self {
        let mut result = match value {
            AST::Abstraction(ASTAbstraction {
                parameters,
                box expression,
            }) => {
                let mut iter = parameters.iter();
                let ASTIdentifier(parameter) = iter.next_back().unwrap();
                Expression::Abstraction(iter.rfold(
                    Abstraction::new(parameter, expression.into()),
                    |body, ASTIdentifier(parameter)| {
                        Abstraction::new(parameter, Expression::Abstraction(body))
                    },
                ))
            }
            AST::Application(ASTApplication { expressions }) => {
                let mut iter = expressions.iter();
                let callee = iter.next().unwrap();
                if let Some(argument) = iter.next() {
                    Expression::Application(iter.fold(
                        Application::new(callee.into(), argument.into()),
                        |callee, argument| {
                            Application::new(Expression::Application(callee), argument.into())
                        },
                    ))
                } else {
                    callee.into()
                }
            }
            AST::Identifier(identifier) => Expression::Variable(identifier.into()),
        };
        result.assign_indices();
        result
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expression::Abstraction(abstraction) => abstraction.fmt(f),
            Expression::Application(application) => application.fmt(f),
            Expression::Variable(variable) => variable.fmt(f),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn translate_abstraction() {
        let a = Expression::from(&AST::Abstraction(ASTAbstraction::new(vec!["x", "x"], "x")));
        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Abstraction(Abstraction::new(
                "x",
                Expression::Variable(Variable::new(Some(0), "x")),
            )),
        ));
        assert_eq!(expected, a);

        let b = Expression::from(&AST::Abstraction(ASTAbstraction::new(
            vec!["x"],
            ASTApplication::new(vec![
                AST::Abstraction(ASTAbstraction::new(vec!["x"], "x")),
                AST::Identifier("x".into()),
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
        let a = Expression::from(&AST::Application(ASTApplication::new(vec!["a", "b", "c"])));
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
        let result = Expression::Variable(Variable::new(Some(0), "x")).shift_impl(true, 0);
        assert_eq!(expected, result);

        let expected = Expression::Variable(Variable::new(Some(0), "x"));
        let result = Expression::Variable(Variable::new(Some(0), "x")).shift_impl(true, 1);
        assert_eq!(expected, result);

        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(2), "y")),
        ));
        let result = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(1), "y")),
        )).shift_impl(true, 0);
        assert_eq!(expected, result);

        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(0), "x")),
        ));
        let result = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(0), "x")),
        )).shift_impl(true, 0);
        assert_eq!(expected, result);

        let expected = Expression::Application(Application::new(
            Expression::Variable(Variable::new(Some(1), "x")),
            Expression::Variable(Variable::new(Some(2), "y")),
        ));
        let result = Expression::Application(Application::new(
            Expression::Variable(Variable::new(Some(0), "x")),
            Expression::Variable(Variable::new(Some(1), "y")),
        )).shift_impl(true, 0);
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
