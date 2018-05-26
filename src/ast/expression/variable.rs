use ast::Identifier;

#[derive(Debug, PartialEq)]
pub struct VariableExpression {
    pub identifier: Identifier,
}

impl VariableExpression {
    pub fn new<T>(identifier: T) -> VariableExpression
    where
        T: Into<Identifier>,
    {
        VariableExpression {
            identifier: identifier.into(),
        }
    }
}

impl<T> From<T> for VariableExpression
where
    T: Into<Identifier>,
{
    fn from(identifier: T) -> VariableExpression {
        VariableExpression::new(identifier)
    }
}
