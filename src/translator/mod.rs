#[cfg(test)]
mod tests;

use parser::ast;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Abstraction {
        name: String,
        expression: Box<Expression>,
    },
    Application {
        callee: Box<Expression>,
        argument: Box<Expression>,
    },
    Variable {
        index: Option<usize>,
        name: String,
    },
}

impl Expression {
    pub fn from_ast(expression: &ast::Expression) -> Expression {
        match expression {
            ast::Expression::Abstraction {
                parameters,
                expression,
            } => Expression::new_abstraction(&parameters, expression),
            ast::Expression::Application { expressions } => {
                Expression::new_application(&expressions)
            }
            ast::Expression::Variable(ast::Variable(name)) => Expression::new_variable(name),
        }
    }

    fn new_abstraction(parameters: &[ast::Variable], expression: &ast::Expression) -> Expression {
        let ast::Variable(name) = &parameters[0];
        Expression::Abstraction {
            name: name.to_owned(),
            expression: box if parameters.len() == 1 {
                Expression::from_ast(expression)
            } else {
                Expression::new_abstraction(&parameters[1..], expression)
            },
        }
    }

    fn new_application(expressions: &[ast::Expression]) -> Expression {
        let argument = expressions.last().unwrap();
        if expressions.len() == 1 {
            Expression::from_ast(argument)
        } else {
            Expression::Application {
                callee: box Expression::new_application(&expressions[..expressions.len() - 1]),
                argument: box Expression::from_ast(argument),
            }
        }
    }

    fn new_variable(name: &str) -> Expression {
        Expression::Variable {
            index: None,
            name: name.to_owned(),
        }
    }
}
