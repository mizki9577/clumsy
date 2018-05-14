use interpreter::Expression;
use parser::ast::{ASTAbstraction, ASTIdentifier, AST};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Abstraction {
    pub name: String,
    pub expression: Box<Expression>,
}

impl Abstraction {
    pub fn new(name: &str, expression: Expression) -> Abstraction {
        Abstraction {
            name: name.to_owned(),
            expression: box expression,
        }
    }

    fn from_ast_impl(parameters: &[ASTIdentifier], expression: &AST) -> Abstraction {
        let ASTIdentifier(name) = &parameters[0];
        Abstraction::new(
            name,
            if parameters.len() == 1 {
                expression.into()
            } else {
                Expression::Abstraction(Abstraction::from_ast_impl(&parameters[1..], expression))
            },
        )
    }
}

impl<'a> From<&'a ASTAbstraction> for Abstraction {
    fn from(value: &ASTAbstraction) -> Self {
        let ASTAbstraction {
            parameters,
            expression,
        } = value;
        Abstraction::from_ast_impl(parameters, expression)
    }
}

impl Display for Abstraction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, r"(\{}. {})", self.name, self.expression)
    }
}
