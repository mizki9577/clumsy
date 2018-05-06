#[cfg(test)]
mod tests;

use parser::ast;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

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
        let mut result = match expression {
            ast::Expression::Abstraction {
                parameters,
                expression,
            } => Expression::new_abstraction(&parameters, expression),
            ast::Expression::Application { expressions } => {
                Expression::new_application(&expressions)
            }
            ast::Expression::Variable(ast::Variable(name)) => Expression::new_variable(name),
        };
        result.assign_indices();
        result
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

    fn assign_indices(&mut self) {
        self.assign_indices_impl(&mut HashMap::new())
    }

    fn assign_indices_impl<'a>(&'a mut self, table: &mut HashMap<&'a str, usize>) {
        match self {
            Expression::Abstraction { name, expression } => {
                table.iter_mut().for_each(|(_, i)| *i += 1); // why rustc tells me `i` does not need to be mutable?
                table.insert(name, 0);
                expression.assign_indices_impl(table);
            }
            Expression::Application { callee, argument } => {
                callee.assign_indices_impl(table);
                argument.assign_indices_impl(table);
            }
            Expression::Variable { name, index } => {
                *index = table.get(name.as_str()).cloned();
            }
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expression::Abstraction { name, expression } => {
                write!(f, r"(\{}. {})", name, expression)
            }
            Expression::Application { callee, argument } => write!(f, r"({} {})", callee, argument),
            Expression::Variable { name, index: _ } => write!(f, r"{}", name),
        }
    }
}
