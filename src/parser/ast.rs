#[derive(Debug, PartialEq)]
pub enum Expression {
    Abstraction {
        variables: Vec<Variable>,
        expression: Box<Expression>,
    },
    Application {
        expressions: Vec<Expression>,
    },
    Variable(Variable),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Variable(pub String);

impl Expression {
    pub fn new_abstraction(variables: Vec<Variable>, expression: Expression) -> Expression {
        Expression::Abstraction {
            variables,
            expression: box expression,
        }
    }

    pub fn new_application(expressions: Vec<Expression>) -> Expression {
        Expression::Application { expressions }
    }

    pub fn new_variable(variable: &str) -> Expression {
        Expression::Variable(Variable::from(variable))
    }
}

impl<T> From<T> for Variable
where
    T: Into<String>,
{
    fn from(value: T) -> Variable {
        Variable(value.into())
    }
}
