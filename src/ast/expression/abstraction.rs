use ast::{Expression, Identifier};

#[derive(Debug, PartialEq)]
pub struct AbstractionExpression {
    pub parameters: Vec<Identifier>,
    pub expression: Box<Expression>,
}

impl AbstractionExpression {
    pub fn new<T, U>(parameters: T, expression: U) -> AbstractionExpression
    where
        T: Into<Vec<Identifier>>,
        U: Into<Expression>,
    {
        AbstractionExpression {
            parameters: parameters.into(),
            expression: box expression.into(),
        }
    }
}
