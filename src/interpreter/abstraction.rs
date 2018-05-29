use ast;
use interpreter::Expression;
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
        let Abstraction { name, expression } = self;

        let outer = table.get(name.as_str()).cloned();
        table.iter_mut().for_each(|(_, i)| *i += 1);
        table.insert(name, 0);

        expression.assign_indices(table);

        table.remove(name.as_str());
        table.iter_mut().for_each(|(_, i)| *i -= 1);
        if let Some(i) = outer {
            table.insert(name, i);
        }
    }

    pub fn applied(self, argument: Expression) -> Expression {
        let Abstraction { expression, .. } = self;
        expression
            .substituted(0, argument.shifted(1, 0))
            .shifted(-1, 0)
    }

    pub fn shifted(self, d: isize, c: usize) -> Self {
        let Abstraction { name, expression } = self;
        Abstraction {
            name,
            expression: box expression.shifted(d, c + 1),
        }
    }

    pub fn substituted(self, j: usize, term: Expression) -> Self {
        let Abstraction { name, expression } = self;
        Abstraction {
            name,
            expression: box expression.substituted(j + 1, term.shifted(1, 0)),
        }
    }
}

impl<'a> From<&'a ast::AbstractionExpression> for Abstraction {
    fn from(value: &ast::AbstractionExpression) -> Abstraction {
        let ast::AbstractionExpression {
            parameters,
            box expression,
        } = value;

        let mut iter = parameters.iter();
        let ast::Identifier(parameter) = iter.next_back().unwrap();

        iter.rfold(
            Abstraction {
                name: parameter.to_owned(),
                expression: box expression.into(),
            },
            |body, ast::Identifier(parameter)| Abstraction {
                name: parameter.to_owned(),
                expression: box Expression::Abstraction(body),
            },
        )
    }
}

impl Display for Abstraction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, r"(\{}. {})", self.name, self.expression)
    }
}
