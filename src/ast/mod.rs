mod expression;
mod program;
mod statement;
pub use self::expression::*;
pub use self::program::*;
pub use self::statement::*;

#[derive(Debug, PartialEq)]
pub struct Identifier(pub String);

impl Identifier {
    fn new<T>(value: T) -> Identifier
    where
        T: Into<String>,
    {
        Identifier(value.into())
    }
}
