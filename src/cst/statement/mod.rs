mod expression;
mod let_s;
pub use self::expression::*;
pub use self::let_s::*;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(ExpressionStatement),
    Let(LetStatement),
}

impl From<ExpressionStatement> for Statement {
    fn from(expression_statement: ExpressionStatement) -> Statement {
        Statement::Expression(expression_statement)
    }
}

impl From<LetStatement> for Statement {
    fn from(let_statement: LetStatement) -> Statement {
        Statement::Let(let_statement)
    }
}
