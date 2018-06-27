mod abstraction;
mod application;
mod variable;
pub use self::abstraction::*;
pub use self::application::*;
pub use self::variable::*;
use cst::Number;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Variable(VariableExpression),
    Abstraction(AbstractionExpression),
    Application(ApplicationExpression),
    Number(Number),
}

impl From<VariableExpression> for Expression {
    fn from(variable: VariableExpression) -> Expression {
        Expression::Variable(variable)
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

impl From<Number> for Expression {
    fn from(number: Number) -> Expression {
        Expression::Number(number)
    }
}
