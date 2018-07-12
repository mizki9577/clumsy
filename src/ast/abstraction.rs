use ast::Expression;
use cst::{AbstractionExpression, Identifier};
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

    pub fn from_cst<'a>(
        value: &'a AbstractionExpression,
        scopes: &mut Vec<&'a str>,
    ) -> Abstraction {
        let mut iter = value.parameters.iter();
        let Identifier(parameter) = iter.next_back().unwrap();

        for Identifier(parameter) in &value.parameters {
            scopes.push(parameter);
        }

        let result = iter.rfold(
            Abstraction::new(
                parameter.as_str(),
                Expression::from_cst(&*value.expression, scopes),
            ),
            |body, Identifier(parameter)| {
                Abstraction::new(parameter.as_str(), Expression::Abstraction(body))
            },
        );

        for _ in &value.parameters {
            scopes.pop();
        }

        result
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
