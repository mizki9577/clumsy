mod abstraction;
mod application;
pub use self::abstraction::*;
pub use self::application::*;

use ast::Identifier;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Variable(Identifier),
    Abstraction(AbstractionExpression),
    Application(ApplicationExpression),
}

impl From<Identifier> for Expression {
    fn from(identifier: Identifier) -> Expression {
        Expression::Variable(identifier)
    }
}

impl From<AbstractionExpression> for Expression {
    fn from(abstraction: AbstractionExpression) -> Expression {
        Expression::Abstraction(abstraction)
    }
}

impl From<ApplicationExpression> for Expression {
    fn from(application: ApplicationExpression) -> Expression {
        Expression::Application(application)
    }
}
