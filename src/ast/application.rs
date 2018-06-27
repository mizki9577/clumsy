use ast::Expression;
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
        self.callee.assign_indices(table);
        self.argument.assign_indices(table);
    }

    pub fn evaluate1(self) -> Expression {
        match self {
            Application {
                callee: box Expression::Abstraction(callee),
                box argument,
            } => callee.applied(argument),

            Application {
                callee: box Expression::Application(callee),
                box argument,
            } => Expression::Application(Application::new(callee.evaluate1(), argument)),

            _ => Expression::Application(self),
        }
    }

    pub fn shifted(self, d: isize, c: usize) -> Application {
        Application::new(self.callee.shifted(d, c), self.argument.shifted(d, c))
    }

    pub fn substituted(self, j: usize, term: Expression) -> Application {
        let cloned_term = term.clone();
        Application::new(
            self.callee.substituted(j, term),
            self.argument.substituted(j, cloned_term),
        )
    }

    pub fn is_reducible(&self) -> bool {
        match *self.callee {
            Expression::Variable(..) => false,
            Expression::Abstraction(..) => true,
            Expression::Application(ref callee) => callee.is_reducible(),
        }
    }
}

impl Display for Application {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self.callee {
            Expression::Variable(ref variable) => variable.fmt(f)?,
            Expression::Abstraction(ref abstraction) => write!(f, r"({})", abstraction)?,
            Expression::Application(ref application) => application.fmt(f)?,
        }

        f.write_str(" ")?;

        match *self.argument {
            Expression::Variable(ref variable) => variable.fmt(f),
            Expression::Abstraction(ref abstraction) => write!(f, r"({})", abstraction),
            Expression::Application(ref application) => write!(f, r"({})", application),
        }
    }
}
