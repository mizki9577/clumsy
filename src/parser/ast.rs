#[derive(Debug, PartialEq)]
pub enum Expression {
    Abstraction {
        parameter: Variable,
        expression: Box<Expression>,
    },
    Application {
        callee: Box<Expression>,
        argument: Box<Expression>,
    },
    Variable(Variable),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Variable(pub String);

impl Expression {
    pub fn new_abstraction(parameter: &str, expression: Expression) -> Expression {
        Expression::Abstraction {
            parameter: Variable::new(parameter),
            expression: box expression,
        }
    }

    pub fn new_application(callee: Expression, argument: Expression) -> Expression {
        Expression::Application {
            callee: box callee,
            argument: box argument,
        }
    }

    pub fn new_variable(variable: &str) -> Expression {
        Expression::Variable(Variable::new(variable))
    }
}

impl Variable {
    pub fn new(variable: &str) -> Variable {
        Variable(variable.to_owned())
    }
}
