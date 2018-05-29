use ast;
use interpreter::Expression;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    name: String,
    index: Option<usize>,
}

impl Variable {
    pub fn new<T>(index: Option<usize>, name: T) -> Variable
    where
        T: Into<String>,
    {
        Variable {
            name: name.into(),
            index,
        }
    }

    pub fn assign_indices<'a>(&'a mut self, table: &mut HashMap<&'a str, usize>) {
        let Variable { name, index } = self;
        *index = table.get(name.as_str()).cloned();
    }

    pub fn shifted(self, d: isize, c: usize) -> Self {
        match self {
            Variable {
                index: Some(index),
                ref name,
            } if index >= c =>
            {
                Variable {
                    index: Some((index as isize + d) as usize),
                    name: name.to_owned(),
                }
            }

            _ => self,
        }
    }

    pub fn substituted(self, j: usize, term: Expression) -> Expression {
        match self {
            Variable {
                index: Some(index), ..
            } if index == j =>
            {
                term
            }

            _ => Expression::Variable(self),
        }
    }
}

impl<'a> From<&'a ast::Identifier> for Variable {
    fn from(value: &ast::Identifier) -> Variable {
        let ast::Identifier(identifier) = value;
        Variable {
            name: identifier.to_owned(),
            index: None,
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, r"{}", self.name)
    }
}
