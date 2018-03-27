pub type Program = Vec<Expression>;

#[derive(Debug)]
pub enum Expression {
    Symbol(Symbol),
    Function(Function),
    Application(Application),
}

#[derive(Debug)]
pub struct Symbol(pub String);

#[derive(Debug)]
pub struct Function {
    pub parameter: Symbol,
    pub body: Box<Expression>,
}

#[derive(Debug)]
pub struct Application {
    pub callee: Box<Expression>,
    pub argument: Box<Expression>,
}
