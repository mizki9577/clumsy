use ast;
use expression::Expression;
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

    pub fn from_ast<'a>(
        value: &'a ast::AbstractionExpression,
        table: &mut HashMap<&'a str, usize>,
    ) -> Abstraction {
        // FIXME super dummy code
        let mut iter = value.parameters.iter();
        let ast::Identifier(parameter) = iter.next_back().unwrap();

        let outer = table.get(parameter.as_str()).cloned();
        table.iter_mut().for_each(|(_, i)| *i += 1);
        table.insert(parameter.as_str(), 0);

        let result = iter.rfold(
            Abstraction::new(
                parameter.as_str(),
                Expression::from_ast(&value.expression, table),
            ),
            |body, ast::Identifier(parameter)| {
                let outer = table.get(parameter.as_str()).cloned();
                table.iter_mut().for_each(|(_, i)| *i += 1);
                table.insert(parameter.as_str(), 0);

                let result = Abstraction::new(parameter.as_str(), Expression::Abstraction(body));

                table.remove(parameter.as_str());
                table.iter_mut().for_each(|(_, i)| *i -= 1);
                if let Some(i) = outer {
                    table.insert(parameter.as_str(), i);
                }

                result
            },
        );

        table.remove(parameter.as_str());
        table.iter_mut().for_each(|(_, i)| *i -= 1);
        if let Some(i) = outer {
            table.insert(parameter.as_str(), i);
        }

        result
    }

    pub fn applied(self, argument: Expression) -> Expression {
        self.expression
            .substituted(0, argument.shifted(1, 0))
            .shifted(-1, 0)
    }

    pub fn shifted(self, d: isize, c: usize) -> Self {
        Abstraction::new(self.name, self.expression.shifted(d, c + 1))
    }

    pub fn substituted(self, j: usize, term: Expression) -> Self {
        Abstraction::new(
            self.name,
            self.expression.substituted(j + 1, term.shifted(1, 0)),
        )
    }
}

impl Display for Abstraction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, r"(\{}. {})", self.name, self.expression)
    }
}
