use parser::ast::*;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Abstraction {
        name: String,
        expression: Box<Expression>,
    },
    Application {
        callee: Box<Expression>,
        argument: Box<Expression>,
    },
    Variable {
        index: Option<usize>,
        name: String,
    },
}

impl Expression {
    fn assign_indices(&mut self) {
        fn assign_indices_impl<'a>(value: &'a mut Expression, table: &mut HashMap<&'a str, usize>) {
            match value {
                Expression::Abstraction { name, expression } => {
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

                Expression::Application { callee, argument } => {
                    assign_indices_impl(callee, table);
                    assign_indices_impl(argument, table);
                }

                Expression::Variable { name, index } => {
                    *index = table.get(name.as_str()).cloned();
                }
            }
        }

        assign_indices_impl(self, &mut HashMap::new())
    }

    pub fn evaluate(self) -> Self {
        fn evaluate1(value: Expression) -> Result<Expression, Expression> {
            match value {
                Expression::Application {
                    callee: box Expression::Abstraction { expression, .. },
                    box argument,
                } => Ok(expression
                    .substituted(0, argument.shifted(1, 0))
                    .shifted(-1, 0)),

                Expression::Application {
                    callee: box callee,
                    box argument,
                } => match evaluate1(callee) {
                    Ok(callee) => Ok(Expression::Application {
                        callee: box callee,
                        argument: box argument,
                    }),
                    Err(callee) => Err(Expression::Application {
                        callee: box callee,
                        argument: box argument,
                    }),
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
            Expression::Abstraction { name, expression } => Expression::Abstraction {
                name,
                expression: box expression.shifted(d, c + 1),
            },

            Expression::Application { callee, argument } => Expression::Application {
                callee: box callee.shifted(d, c),
                argument: box argument.shifted(d, c),
            },

            Expression::Variable {
                index: Some(index),
                ref name,
            } if index >= c =>
            {
                Expression::Variable {
                    index: Some((index as isize + d) as usize),
                    name: name.to_owned(),
                }
            }

            _ => self,
        }
    }

    fn substituted(self, j: usize, term: Expression) -> Self {
        match self {
            Expression::Abstraction { name, expression } => Expression::Abstraction {
                name,
                expression: box expression.substituted(j + 1, term.shifted(1, 0)),
            },
            Expression::Application { callee, argument } => {
                let cloned_term = term.clone();
                Expression::Application {
                    callee: box callee.substituted(j, term),
                    argument: box argument.substituted(j, cloned_term),
                }
            }
            Expression::Variable {
                index: Some(index), ..
            } if index == j =>
            {
                term
            }
            _ => self,
        }
    }
}

impl<'a> From<&'a ASTExpression> for Expression {
    fn from(value: &ASTExpression) -> Self {
        let mut result = match value {
            ASTExpression::Abstraction(ASTAbstraction {
                parameters,
                box expression,
            }) => {
                let mut iter = parameters.iter();
                let ASTIdentifier(parameter) = iter.next_back().unwrap();
                iter.rfold(
                    Expression::Abstraction {
                        name: parameter.to_owned(),
                        expression: box expression.into(),
                    },
                    |body, ASTIdentifier(parameter)| Expression::Abstraction {
                        name: parameter.to_owned(),
                        expression: box body,
                    },
                )
            }

            ASTExpression::Application(ASTApplication { expressions }) => {
                let mut iter = expressions.iter();
                let callee = iter.next().unwrap();
                if let Some(argument) = iter.next() {
                    iter.fold(
                        Expression::Application {
                            callee: box callee.into(),
                            argument: box argument.into(),
                        },
                        |callee, argument| Expression::Application {
                            callee: box callee,
                            argument: box argument.into(),
                        },
                    )
                } else {
                    callee.into()
                }
            }

            ASTExpression::Identifier(ASTIdentifier(identifier)) => Expression::Variable {
                name: identifier.to_owned(),
                index: None,
            },
        };
        result.assign_indices();
        result
    }
}

impl<'a> From<&'a ASTProgram> for Expression {
    fn from(value: &ASTProgram) -> Self {
        let ASTProgram(directives) = value;

        let mut iter = directives.iter().rev();
        if let Some(ASTDirective::Expression(result)) = iter.next() {
            let mut result = iter.fold(result.into(), |result, directive| match directive {
                ASTDirective::Expression(..) => unimplemented!(),
                ASTDirective::Let(ASTLet {
                    variable: ASTIdentifier(variable),
                    box expression,
                }) => Expression::Application {
                    callee: box Expression::Abstraction {
                        name: variable.to_owned(),
                        expression: box result,
                    },
                    argument: box expression.into(),
                },
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
            Expression::Abstraction { name, expression } => {
                write!(f, r"(\{}. {})", name, expression)
            }
            Expression::Application { callee, argument } => write!(f, r"({} {})", callee, argument),
            Expression::Variable { name, .. } => write!(f, r"{}", name),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn translate_abstraction() {
        let a = Expression::from(&ASTExpression::Abstraction(ASTAbstraction::new(
            vec!["x", "x"],
            "x",
        )));
        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Abstraction(Abstraction::new(
                "x",
                Expression::Variable(Variable::new(Some(0), "x")),
            )),
        ));
        assert_eq!(expected, a);

        let b = Expression::from(&ASTExpression::Abstraction(ASTAbstraction::new(
            vec!["x"],
            ASTApplication::new(vec![
                ASTExpression::Abstraction(ASTAbstraction::new(vec!["x"], "x")),
                ASTExpression::Identifier("x".into()),
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
        let a = Expression::from(&ASTExpression::Application(ASTApplication::new(vec![
            "a", "b", "c",
        ])));
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
