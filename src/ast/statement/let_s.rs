use ast::{Expression, Identifier};

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub variable: Identifier,
    pub expression: Expression,
}

impl LetStatement {
    fn new<T, U>(variable: T, expression: U) -> LetStatement
    where
        T: Into<Identifier>,
        U: Into<Expression>,
    {
        LetStatement {
            variable: variable.into(),
            expression: expression.into(),
        }
    }
}
