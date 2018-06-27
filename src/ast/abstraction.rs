use ast::Expression;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct Abstraction {
    name: String,
    expression: Box<Expression>,
}

impl Abstraction {
    pub fn new<T, U>(name: T, expression: U) -> Abstraction
    where
        T: Into<String>,
        U: Into<Expression>,
    {
        Abstraction {
            name: name.into(),
            expression: box expression.into(),
        }
    }

    pub fn assign_indices<'a>(&'a mut self, table: &mut HashMap<&'a str, usize>) {
        let outer = table.get(self.name.as_str()).cloned();
        table.iter_mut().for_each(|(_, i)| *i += 1);
        table.insert(&self.name, 0);

        self.expression.assign_indices(table);

        table.remove(self.name.as_str());
        table.iter_mut().for_each(|(_, i)| *i -= 1);
        if let Some(i) = outer {
            table.insert(&self.name, i);
        }
    }

    pub fn applied(self, argument: Expression) -> Expression {
        self.expression
            .substituted(0, argument.shifted(1, 0))
            .shifted(-1, 0)
    }

    pub fn shifted(self, d: isize, c: usize) -> Abstraction {
        Abstraction::new(self.name, self.expression.shifted(d, c + 1))
    }

    pub fn substituted(self, j: usize, term: Expression) -> Abstraction {
        Abstraction::new(
            self.name,
            self.expression.substituted(j + 1, term.shifted(1, 0)),
        )
    }
}

impl Display for Abstraction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self.expression {
            Expression::Variable(ref variable) => write!(f, r"\{}. {}", self.name, variable),
            Expression::Abstraction(ref abstraction) => {
                write!(f, r"\{} {}", self.name, &abstraction.to_string()[1..])
            }
            Expression::Application(ref application) => {
                write!(f, r"\{}. {}", self.name, application)
            }
        }
    }
}
