use interpreter::Expression;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Abstraction {
    pub name: String,
    pub expression: Box<Expression>,
}

impl Abstraction {
    pub fn new(name: &str, expression: Expression) -> Abstraction {
        Abstraction {
            name: name.to_owned(),
            expression: box expression,
        }
    }
}

impl Display for Abstraction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, r"(\{}. {})", self.name, self.expression)
    }
}
