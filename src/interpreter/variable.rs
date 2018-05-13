use parser::ast;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
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

impl<'a> From<&'a ast::Variable> for Variable {
    fn from(value: &ast::Variable) -> Variable {
        let ast::Variable(name) = value;
        Variable::new(None, name)
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, r"{}", self.name)
    }
}
