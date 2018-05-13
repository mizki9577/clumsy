use interpreter::Expression;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Application {
    pub callee: Box<Expression>,
    pub argument: Box<Expression>,
}

impl Application {
    pub fn new(callee: Expression, argument: Expression) -> Application {
        Application {
            callee: box callee,
            argument: box argument,
        }
    }
}

impl Display for Application {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, r"({} {})", self.callee, self.argument)
    }
}
