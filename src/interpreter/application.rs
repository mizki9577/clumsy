use ast;
use interpreter::Expression;

#[derive(Debug, PartialEq, Clone)]
pub struct Application {
    pub callee: Box<Expression>,
    pub argument: Box<Expression>,
}

impl Application {
    pub fn new<T, U>(callee: T, argument: U) -> Application
    where
        T: Into<Expression>,
        U: Into<Expression>,
    {
        Application {
            callee: box callee.into(),
            argument: box argument.into(),
        }
    }
}

impl<'a> From<&'a ast::ApplicationExpression> for Expression {
    fn from(value: &ast::ApplicationExpression) -> Expression {
        let ast::ApplicationExpression { expressions } = value;

        let mut iter = expressions.iter();
        let callee = iter.next().unwrap();

        if let Some(argument) = iter.next() {
            iter.fold(
                Expression::Application(Application {
                    callee: box callee.into(),
                    argument: box argument.into(),
                }),
                |callee, argument| {
                    Expression::Application(Application {
                        callee: box callee,
                        argument: box argument.into(),
                    })
                },
            )
        } else {
            callee.into()
        }
    }
}
