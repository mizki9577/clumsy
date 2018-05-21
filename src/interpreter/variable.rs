use parser::ast::ASTIdentifier;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub index: Option<usize>,
    pub name: String,
}

impl Variable {
    pub fn new(index: Option<usize>, name: &str) -> Variable {
        Variable {
            index,
            name: name.to_owned(),
        }
    }
}

impl<'a> From<&'a ASTIdentifier> for Variable {
    fn from(value: &ASTIdentifier) -> Self {
        let ASTIdentifier(name) = value;
        Variable::new(None, name)
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, r"{}:{}", self.name, self.index.unwrap_or(42))
    }
}
