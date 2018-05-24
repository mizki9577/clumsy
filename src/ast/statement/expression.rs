use ast::Expression;

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

impl ExpressionStatement {
    fn new<T>(expression: T) -> ExpressionStatement
    where
        T: Into<Expression>,
    {
        ExpressionStatement {
            expression: expression.into(),
        }
    }
}
