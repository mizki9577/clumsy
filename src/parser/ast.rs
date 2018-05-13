#[derive(Debug, PartialEq)]
pub enum Expression {
    Abstraction(Abstraction),
    Application(Application),
    Variable(Variable),
}

#[derive(Debug, PartialEq)]
pub struct Abstraction {
    pub parameters: Vec<Variable>,
    pub expression: Box<Expression>,
}

impl Abstraction {
    pub fn new(
        parameters: impl IntoIterator<Item = Variable>,
        expression: Expression,
    ) -> Abstraction {
        Abstraction {
            parameters: parameters.into_iter().collect(),
            expression: box expression,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Application {
    pub expressions: Vec<Expression>,
}

impl Application {
    pub fn new(expressions: impl IntoIterator<Item = Expression>) -> Application {
        Application {
            expressions: expressions.into_iter().collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Variable(pub String);

impl<T> From<T> for Variable
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Variable(value.into())
    }
}
