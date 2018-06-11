use ast;
use expression::Expression;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct Application {
    callee: Box<Expression>,
    argument: Box<Expression>,
}

impl Application {
    pub fn new<T, U>(callee: T, argument: U) -> Application
    where
        T: Into<Expression>,
        U: Into<Expression>,
    {
        Application {
            callee: box callee.into(),
            argument: box argument.into(),
        }
    }

    pub fn assign_indices<'a>(&'a mut self, table: &mut HashMap<&'a str, usize>) {
        let Application { callee, argument } = self;
        callee.assign_indices(table);
        argument.assign_indices(table);
    }

    pub fn evaluate1(self) -> Expression {
        match self {
            Application {
                callee: box Expression::Abstraction(callee),
                box argument,
            } => callee.applied(argument),

            Application {
                callee: box Expression::Application(callee),
                argument,
            } => Expression::Application(Application {
                callee: box callee.evaluate1(),
                argument,
            }),

            _ => Expression::Application(self),
        }
    }

    pub fn shifted(self, d: isize, c: usize) -> Self {
        let Application { callee, argument } = self;
        Application {
            callee: box callee.shifted(d, c),
            argument: box argument.shifted(d, c),
        }
    }

    pub fn substituted(self, j: usize, term: Expression) -> Self {
        let Application { callee, argument } = self;
        let cloned_term = term.clone();
        Application {
            callee: box callee.substituted(j, term),
            argument: box argument.substituted(j, cloned_term),
        }
    }
}

impl<'a> From<&'a ast::ApplicationExpression> for Expression {
    fn from(value: &ast::ApplicationExpression) -> Expression {
        let ast::ApplicationExpression { expressions } = value;

        let mut iter = expressions.iter();
        let callee = iter.next().unwrap();

        if let Some(argument) = iter.next() {
            iter.fold(
                Expression::Application(Application {
                    callee: box callee.into(),
                    argument: box argument.into(),
                }),
                |callee, argument| {
                    Expression::Application(Application {
                        callee: box callee,
                        argument: box argument.into(),
                    })
                },
            )
        } else {
            callee.into()
        }
    }
}

impl Display for Application {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, r"({} {})", self.callee, self.argument)
    }
}
