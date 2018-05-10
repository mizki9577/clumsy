use translator::Expression;

#[derive(Debug, PartialEq)]
pub struct Application {
    pub callee: Box<Expression>,
    pub argument: Box<Expression>,
}

impl Application {
    pub fn new(callee: Expression, argument: Expression) -> Application {
        Application {
            callee: box callee,
            argument: box argument,
        }
    }
}
