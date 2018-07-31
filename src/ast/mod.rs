use cst::{Expression as CSTExpression, *};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Variable {
        index: Option<usize>,
    },
    Abstraction {
        expression: Box<Expression>,
    },
    Application {
        applicand: Box<Expression>,
        argument: Box<Expression>,
    },
}

impl Expression {
    pub fn from_cst_program(value: &Program) -> Expression {
        let Program(statements) = value;

        let mut iter = statements.iter();

        let innermost = if let Some(Statement::Expression(ExpressionStatement { expression })) =
            iter.next_back()
        {
            expression
        } else {
            unimplemented!()
        };

        let mut scopes = iter
            .clone()
            .filter_map(|statement| match statement {
                Statement::Let(LetStatement {
                    variable: Identifier(variable),
                    ..
                }) => Some(variable.as_str()),
                _ => None,
            })
            .collect();

        let mut outer_scope = Vec::new();
        let result = iter.rfold(
            Expression::from_cst_expression(innermost, &mut scopes),
            |inner, statement| match statement {
                Statement::Expression(..) => unimplemented!(),
                Statement::Let(LetStatement { expression, .. }) => {
                    outer_scope.push(scopes.pop().unwrap());
                    Expression::Application {
                        applicand: box Expression::Abstraction {
                            expression: box inner,
                        },
                        argument: box Expression::from_cst_expression(expression, &mut scopes),
                    }
                }
            },
        );
        outer_scope
            .into_iter()
            .rfold((), |_, variable| scopes.push(variable));
        result
    }

    fn from_cst_expression<'a>(value: &'a CSTExpression, scopes: &mut Vec<&'a str>) -> Expression {
        match value {
            CSTExpression::Variable(variable) => Expression::variable_from_cst(variable, scopes),

            CSTExpression::Abstraction(abstraction) => {
                Expression::abstraction_from_cst(abstraction, scopes)
            }

            CSTExpression::Application(application) => match application.expressions.len() {
                0 => panic!(),
                1 => Expression::from_cst_expression(&application.expressions[0], scopes),
                _ => Expression::application_from_cst(application, scopes),
            },

            CSTExpression::Number(number) => Expression::from_number(number),

            CSTExpression::Character(character) => Expression::from_character(character),
        }
    }

    fn variable_from_cst(value: &VariableExpression, scopes: &mut Vec<&str>) -> Expression {
        let VariableExpression {
            identifier: Identifier(identifier),
        } = value;

        let index = scopes
            .iter()
            .rposition(|variable| variable == identifier)
            .map(|index| scopes.len() - index - 1);

        Expression::Variable { index }
    }

    fn abstraction_from_cst<'a>(
        value: &'a AbstractionExpression,
        scopes: &mut Vec<&'a str>,
    ) -> Expression {
        for Identifier(parameter) in &value.parameters {
            scopes.push(parameter);
        }

        let result = value.parameters.iter().skip(1).rfold(
            Expression::Abstraction {
                expression: box Expression::from_cst_expression(&*value.expression, scopes),
            },
            |body, _| Expression::Abstraction {
                expression: box body,
            },
        );

        for _ in &value.parameters {
            scopes.pop();
        }

        result
    }

    fn application_from_cst<'a>(
        value: &'a ApplicationExpression,
        scopes: &mut Vec<&'a str>,
    ) -> Expression {
        let mut iter = value.expressions.iter();
        let callee = iter.next().unwrap();
        let argument = iter.next().unwrap();
        iter.fold(
            Expression::Application {
                applicand: box Expression::from_cst_expression(callee, scopes),
                argument: box Expression::from_cst_expression(argument, scopes),
            },
            |callee, argument| Expression::Application {
                applicand: box callee,
                argument: box Expression::from_cst_expression(argument, scopes),
            },
        )
    }

    fn from_number(value: &Number) -> Expression {
        let Number(value) = value;
        let mut n = value.parse::<usize>().unwrap(); // TODO: handle this
        let mut result = Expression::Variable { index: Some(0) };

        while n > 0 {
            result = Expression::Application {
                applicand: box Expression::Variable { index: Some(1) },
                argument: box result,
            };
            n -= 1;
        }

        Expression::Abstraction {
            expression: box Expression::Abstraction {
                expression: box result,
            },
        }
    }

    fn from_character(value: &Character) -> Expression {
        let Character(value) = value;
        let mut n = *value as u32;
        let mut result = Expression::Variable { index: Some(0) };

        while n > 0 {
            result = Expression::Application {
                applicand: box Expression::Variable { index: Some(1) },
                argument: box result,
            };
            n -= 1;
        }

        Expression::Abstraction {
            expression: box Expression::Abstraction {
                expression: box result,
            },
        }
    }

    pub fn is_reducible(&self) -> bool {
        match self {
            Expression::Variable { .. } => false,
            Expression::Abstraction { .. } => false,
            Expression::Application {
                applicand: box Expression::Variable { .. },
                ..
            } => false,
            Expression::Application {
                applicand: box Expression::Abstraction { .. },
                ..
            } => true,
            Expression::Application {
                applicand: box Expression::Application { applicand, .. },
                ..
            } => applicand.is_reducible(),
        }
    }

    pub fn evaluate(mut self) -> Expression {
        while self.is_reducible() {
            self = self.evaluate1();
        }
        self
    }

    fn evaluate1(self) -> Expression {
        match self {
            Expression::Application {
                applicand: box Expression::Abstraction { expression },
                box argument,
            } => expression
                .substituted(0, argument.shifted(1, 0))
                .shifted(-1, 0),

            Expression::Application {
                applicand,
                argument,
            } => Expression::Application {
                applicand: box applicand.evaluate1(),
                argument,
            },

            _ => self,
        }
    }

    fn shifted(self, d: isize, c: usize) -> Expression {
        match self {
            Expression::Variable { index: Some(index) } if index >= c => Expression::Variable {
                index: Some((index as isize + d) as usize),
            },
            Expression::Variable { .. } => self,

            Expression::Abstraction { expression } => Expression::Abstraction {
                expression: box expression.shifted(d, c + 1),
            },

            Expression::Application {
                applicand,
                argument,
            } => Expression::Application {
                applicand: box applicand.shifted(d, c),
                argument: box argument.shifted(d, c),
            },
        }
    }

    fn substituted(self, j: usize, term: Expression) -> Expression {
        match self {
            Expression::Variable { index: Some(index) } if index == j => term,
            Expression::Variable { .. } => self,

            Expression::Abstraction { expression } => Expression::Abstraction {
                expression: box expression.substituted(j + 1, term.shifted(1, 0)),
            },

            Expression::Application {
                applicand,
                argument,
            } => Expression::Application {
                applicand: box applicand.substituted(j, term.clone()),
                argument: box argument.substituted(j, term),
            },
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expression::Variable { index } => match index {
                Some(index) => index.fmt(f),
                None => f.write_str("None"),
            },

            Expression::Abstraction { box expression } => match expression {
                Expression::Variable { .. } => write!(f, r"\ {}", expression),
                Expression::Abstraction { .. } => write!(f, r"\ {}", expression),
                Expression::Application { .. } => write!(f, r"\ {}", expression),
            },

            Expression::Application {
                box applicand,
                box argument,
            } => {
                match applicand {
                    Expression::Variable { .. } => applicand.fmt(f)?,
                    Expression::Abstraction { .. } => write!(f, r"({})", applicand)?,
                    Expression::Application { .. } => applicand.fmt(f)?,
                }

                f.write_str(" ")?;

                match argument {
                    Expression::Variable { .. } => argument.fmt(f),
                    Expression::Abstraction { .. } => write!(f, r"({})", argument),
                    Expression::Application { .. } => write!(f, r"({})", argument),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cst::{
        AbstractionExpression, ApplicationExpression, Expression as CSTExpression,
        ExpressionStatement, Identifier, LetStatement, Program, Statement, VariableExpression,
    };

    #[test]
    fn test_shift() {
        let expected = Expression::Variable { index: Some(1) };
        let result = Expression::Variable { index: Some(0) }.shifted(1, 0);
        assert_eq!(expected, result);

        let expected = Expression::Variable { index: Some(0) };
        let result = Expression::Variable { index: Some(0) }.shifted(1, 1);
        assert_eq!(expected, result);

        let expected = Expression::Abstraction {
            expression: box Expression::Variable { index: Some(2) },
        };
        let result = Expression::Abstraction {
            expression: box Expression::Variable { index: Some(1) },
        }.shifted(1, 0);
        assert_eq!(expected, result);

        let expected = Expression::Abstraction {
            expression: box Expression::Variable { index: Some(0) },
        };
        let result = Expression::Abstraction {
            expression: box Expression::Variable { index: Some(0) },
        }.shifted(1, 0);
        assert_eq!(expected, result);

        let expected = Expression::Application {
            applicand: box Expression::Variable { index: Some(1) },
            argument: box Expression::Variable { index: Some(2) },
        };
        let result = Expression::Application {
            applicand: box Expression::Variable { index: Some(0) },
            argument: box Expression::Variable { index: Some(1) },
        }.shifted(1, 0);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_substitute() {
        let expected = Expression::Variable { index: None };
        let result = Expression::Variable { index: Some(0) }
            .substituted(0, Expression::Variable { index: None });
        assert_eq!(expected, result);

        let expected = Expression::Variable { index: Some(1) };
        let result = Expression::Variable { index: Some(1) }
            .substituted(0, Expression::Variable { index: None });
        assert_eq!(expected, result);

        let expected = Expression::Abstraction {
            expression: box Expression::Variable { index: None },
        };
        let result = Expression::Abstraction {
            expression: box Expression::Variable { index: Some(1) },
        }.substituted(0, Expression::Variable { index: None });
        assert_eq!(expected, result);

        let expected = Expression::Abstraction {
            expression: box Expression::Variable { index: Some(0) },
        };
        let result = Expression::Abstraction {
            expression: box Expression::Variable { index: Some(0) },
        }.substituted(0, Expression::Variable { index: None });
        assert_eq!(expected, result);

        let expected = Expression::Application {
            applicand: box Expression::Variable { index: Some(0) },
            argument: box Expression::Variable { index: None },
        };
        let result = Expression::Application {
            applicand: box Expression::Variable { index: Some(0) },
            argument: box Expression::Variable { index: Some(1) },
        }.substituted(1, Expression::Variable { index: None });
        assert_eq!(expected, result);
    }

    #[test]
    fn translate_abstraction() {
        let result = Expression::from_cst_expression(
            &CSTExpression::from(AbstractionExpression::new(
                vec![Identifier::new("x"), Identifier::new("x")],
                VariableExpression::new(Identifier::new("x")),
            )),
            &mut Vec::new(),
        );

        let expected = Expression::Abstraction {
            expression: box Expression::Abstraction {
                expression: box Expression::Variable { index: Some(0) },
            },
        };
        assert_eq!(expected, result);

        let b = Expression::from_cst_expression(
            &CSTExpression::from(AbstractionExpression::new(
                vec![Identifier::new("x")],
                ApplicationExpression::new(vec![
                    CSTExpression::from(AbstractionExpression::new(
                        vec![Identifier::new("x")],
                        VariableExpression::new(Identifier::new("x")),
                    )),
                    CSTExpression::from(VariableExpression::new(Identifier::new("x"))),
                ]),
            )),
            &mut Vec::new(),
        );
        let expected = Expression::Abstraction {
            expression: box Expression::Application {
                applicand: box Expression::Abstraction {
                    expression: box Expression::Variable { index: Some(0) },
                },
                argument: box Expression::Variable { index: Some(0) },
            },
        };
        assert_eq!(expected, b);
    }

    #[test]
    fn translate_application() {
        let a = Expression::from_cst_expression(
            &CSTExpression::from(ApplicationExpression::new(vec![
                CSTExpression::from(VariableExpression::new(Identifier::new("a"))),
                CSTExpression::from(VariableExpression::new(Identifier::new("b"))),
                CSTExpression::from(VariableExpression::new(Identifier::new("c"))),
            ])),
            &mut Vec::new(),
        );
        let expected = Expression::Application {
            applicand: box Expression::Application {
                applicand: box Expression::Variable { index: None },
                argument: box Expression::Variable { index: None },
            },
            argument: box Expression::Variable { index: None },
        };
        assert_eq!(expected, a);
    }

    #[test]
    fn translate_let_statement() {
        let expected = Expression::Application {
            applicand: box Expression::Abstraction {
                expression: box Expression::Variable { index: Some(0) },
            },
            argument: box Expression::Abstraction {
                expression: box Expression::Variable { index: Some(0) },
            },
        };
        let result = Expression::from_cst_program(&Program(vec![
            Statement::from(LetStatement::new(
                Identifier::new("id"),
                CSTExpression::from(AbstractionExpression::new(
                    vec![Identifier::new("x")],
                    CSTExpression::from(ApplicationExpression::new(vec![CSTExpression::from(
                        VariableExpression::new(Identifier::new("x")),
                    )])),
                )),
            )),
            Statement::from(ExpressionStatement::new(CSTExpression::from(
                VariableExpression::new(Identifier::new("id")),
            ))),
        ]));
        assert_eq!(expected, result);
    }
}
