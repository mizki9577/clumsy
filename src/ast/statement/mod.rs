mod expression;
mod let_s;
pub use self::expression::*;
pub use self::let_s::*;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(ExpressionStatement),
    LetStatement(LetStatement),
}
