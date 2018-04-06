pub type Program = Vec<Expression>;

#[derive(Debug)]
pub enum Expression {
    Abstraction(Abstraction),
    Application(Application),
}

#[derive(Debug)]
pub struct Abstraction {
    pub variables: Vec<Variable>,
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct Application {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    Variable(Variable),
    Expression(Expression),
}

pub type Variables = Vec<Variable>;

#[derive(Debug)]
pub struct Variable(pub String);
