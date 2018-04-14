pub type Program = Vec<Expression>;

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

#[derive(Debug, PartialEq)]
pub struct Variable(pub String);
