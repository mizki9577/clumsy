use ast;
use expression::Expression;
use std::collections::HashMap;
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
            name: name.into(),
            index: index.into(),
        }
    }

    pub fn from_ast<'a>(
        value: &ast::VariableExpression,
        table: &mut HashMap<&'a str, usize>,
    ) -> Variable {
        let ast::VariableExpression {
            identifier: ast::Identifier(name),
        } = value;
        let index = table.get(name.as_str()).cloned();
        Variable::new(index, name.as_str())
    }

    pub fn shifted(self, d: isize, c: usize) -> Self {
        match self {
            Variable {
                index: Some(index),
                ref name,
            } if index >= c =>
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
            } if index == j =>
            {
                term
            }

            _ => Expression::Variable(self),
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, r"{}", self.name)
    }
}
