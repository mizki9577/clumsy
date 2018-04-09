pub type Program = Vec<Expression>;

#[derive(Debug)]
pub enum Expression {
    Abstraction {
        variables: Vec<Variable>,
        expression: Box<Expression>,
    },
    Application {
        items: Vec<Item>,
    },
}

#[derive(Debug)]
pub enum Item {
    Variable(Variable),
    Expression(Expression),
}

pub type Variables = Vec<Variable>;

#[derive(Debug)]
pub struct Variable(pub String);
