use ast;
use interpreter::Expression;

#[derive(Debug, PartialEq, Clone)]
pub struct Abstraction {
    pub name: String,
    pub expression: Box<Expression>,
}

impl Abstraction {
    pub fn new<T, U>(name: T, expression: U) -> Abstraction
    where
        T: Into<String>,
        U: Into<Expression>,
    {
        Abstraction {
            name: name.into(),
            expression: box expression.into(),
        }
    }
}

impl<'a> From<&'a ast::AbstractionExpression> for Abstraction {
    fn from(value: &ast::AbstractionExpression) -> Abstraction {
        let ast::AbstractionExpression {
            parameters,
            box expression,
        } = value;

        let mut iter = parameters.iter();
        let ast::Identifier(parameter) = iter.next_back().unwrap();

        iter.rfold(
            Abstraction {
                name: parameter.to_owned(),
                expression: box expression.into(),
            },
            |body, ast::Identifier(parameter)| Abstraction {
                name: parameter.to_owned(),
                expression: box Expression::Abstraction(body),
            },
        )
    }
}
