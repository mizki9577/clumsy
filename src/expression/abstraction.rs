use ast;
use expression::{Application, Expression, Variable};
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

impl<'a> From<&'a ast::AbstractionExpression> for Abstraction {
    fn from(value: &ast::AbstractionExpression) -> Abstraction {
        let mut iter = value.parameters.iter();
        let ast::Identifier(parameter) = iter.next_back().unwrap();

        iter.rfold(
            Abstraction::new(parameter.as_str(), &*value.expression),
            |body, ast::Identifier(parameter)| {
                Abstraction::new(parameter.as_str(), Expression::Abstraction(body))
            },
        )
    }
}

impl<'a> From<&'a ast::Number> for Abstraction {
    fn from(value: &ast::Number) -> Abstraction {
        let ast::Number(value) = value;
        let mut n = value.parse::<usize>().unwrap(); // TODO: handle this
        let mut result = Expression::Variable(Variable::new(0, "x"));

        while n > 0 {
            result = Expression::Application(Application::new(
                Expression::Variable(Variable::new(1, "f")),
                result,
            ));
            n -= 1;
        }

        Abstraction::new("f", Expression::Abstraction(Abstraction::new("x", result)))
    }
}

impl Display for Abstraction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, r"(\{}. {})", self.name, self.expression)
    }
}
