use ast::Expression;
use cst::{Identifier, VariableExpression};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    name: String,
    index: Option<usize>,
}

impl Variable {
    pub fn new<T, U>(index: T, name: U) -> Variable
    where
        T: Into<Option<usize>>,
        U: Into<String>,
    {
        Variable {
            index: index.into(),
            name: name.into(),
        }
    }

    pub fn from_cst(value: &VariableExpression, scopes: &mut Vec<&str>) -> Variable {
        let VariableExpression {
            identifier: Identifier(identifier),
        } = value;

        let index = scopes
            .iter()
            .rposition(|variable| variable == identifier)
            .map(|index| scopes.len() - index - 1);

        Variable::new(index, identifier.as_str())
    }

    pub fn shifted(self, d: isize, c: usize) -> Variable {
        match self {
            Variable {
                index: Some(index),
                ref name,
            }
                if index >= c =>
            {
                Variable::new((index as isize + d) as usize, name.as_str())
            }

            _ => self,
        }
    }

    pub fn substituted(self, j: usize, term: Expression) -> Expression {
        match self {
            Variable {
                index: Some(index), ..
            }
                if index == j =>
            {
                term
            }

            _ => Expression::Variable(self),
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.name)
    }
}
