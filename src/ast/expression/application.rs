use ast::Expression;

#[derive(Debug, PartialEq)]
pub struct ApplicationExpression {
    pub expressions: Vec<Expression>,
}

impl ApplicationExpression {
    fn new<T>(expressions: T) -> ApplicationExpression
    where
        T: Into<Vec<Expression>>,
    {
        ApplicationExpression {
            expressions: expressions.into(),
        }
    }
}
