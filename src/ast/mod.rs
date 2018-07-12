use cst::{
    AbstractionExpression, ApplicationExpression, Expression as CSTExpression, Identifier,
    VariableExpression,
};
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
    pub fn from_cst<'a>(value: &'a CSTExpression, scopes: &mut Vec<&'a str>) -> Expression {
        match value {
            CSTExpression::Variable(variable) => Expression::variable_from_cst(variable, scopes),

            CSTExpression::Abstraction(abstraction) => {
                Expression::abstraction_from_cst(abstraction, scopes)
            }

            CSTExpression::Application(application) => {
                if application.expressions.len() > 1 {
                    Expression::application_from_cst(application, scopes)
                } else if application.expressions.len() == 1 {
                    Expression::from_cst(&application.expressions[0], scopes)
                } else {
                    panic!()
                }
            }

            CSTExpression::Number(number) => Expression::from(number),

            CSTExpression::Character(character) => Expression::from(character),
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
        let mut iter = value.parameters.iter();
        iter.next_back();

        for Identifier(parameter) in &value.parameters {
            scopes.push(parameter);
        }

        let result = iter.rfold(
            Expression::Abstraction {
                expression: box Expression::from_cst(&*value.expression, scopes),
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
                applicand: box Expression::from_cst(callee, scopes),
                argument: box Expression::from_cst(argument, scopes),
            },
            |callee, argument| Expression::Application {
                applicand: box callee,
                argument: box Expression::from_cst(argument, scopes),
            },
        )
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
}
