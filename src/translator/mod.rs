#[cfg(test)]
mod tests;

use parser::ast;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Abstraction(Abstraction),
    Application(Application),
    Variable(Variable),
}

impl Expression {
    fn assign_indices(&mut self) {
        self.assign_indices_impl(&mut HashMap::new())
    }

    fn assign_indices_impl<'a>(&'a mut self, table: &mut HashMap<&'a str, usize>) {
        match self {
            Expression::Abstraction(Abstraction { name, expression }) => {
                table.iter_mut().for_each(|(_, i)| *i += 1); // why rustc tells me `i` does not need to be mutable?
                table.insert(name, 0);
                expression.assign_indices_impl(table);
            }
            Expression::Application(Application { callee, argument }) => {
                callee.assign_indices_impl(table);
                argument.assign_indices_impl(table);
            }
            Expression::Variable(Variable { name, index }) => {
                *index = table.get(name.as_str()).cloned();
            }
        }
    }

    fn from_ast_impl(expressions: &[ast::Expression]) -> Expression {
        let argument = expressions.last().unwrap();
        if expressions.len() == 1 {
            argument.into()
        } else {
            Expression::Application(Application::new(
                Expression::from_ast_impl(&expressions[..expressions.len() - 1]),
                argument.into(),
            ))
        }
    }
}

impl<'a> From<&'a ast::Expression> for Expression {
    fn from(value: &ast::Expression) -> Expression {
        let mut result = match value {
            ast::Expression::Abstraction(abstraction) => {
                Expression::Abstraction(abstraction.into())
            }
            ast::Expression::Application(application) => application.into(),
            ast::Expression::Variable(variable) => Expression::Variable(variable.into()),
        };
        result.assign_indices();
        result
    }
}

impl<'a> From<&'a ast::Application> for Expression {
    fn from(value: &ast::Application) -> Expression {
        let ast::Application { expressions } = value;
        Expression::from_ast_impl(expressions)
    }
}

impl From<Abstraction> for Expression {
    fn from(value: Abstraction) -> Expression {
        Expression::Abstraction(value)
    }
}

impl From<Application> for Expression {
    fn from(value: Application) -> Expression {
        Expression::Application(value)
    }
}

impl From<Variable> for Expression {
    fn from(value: Variable) -> Expression {
        Expression::Variable(value)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expression::Abstraction(Abstraction { name, expression }) => {
                write!(f, r"(\{}. {})", name, expression)
            }
            Expression::Application(Application { callee, argument }) => {
                write!(f, r"({} {})", callee, argument)
            }
            Expression::Variable(Variable { name, index: _ }) => write!(f, r"{}", name),
        }
    }
}

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

    fn from_ast_impl(parameters: &[ast::Variable], expression: &ast::Expression) -> Abstraction {
        let ast::Variable(name) = &parameters[0];
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

impl<'a> From<&'a ast::Abstraction> for Abstraction {
    fn from(value: &ast::Abstraction) -> Abstraction {
        let ast::Abstraction {
            parameters,
            expression,
        } = value;
        Abstraction::from_ast_impl(parameters, expression)
    }
}

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

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub index: Option<usize>,
    pub name: String,
}

impl Variable {
    pub fn new(index: Option<usize>, name: &str) -> Variable {
        Variable {
            index,
            name: name.to_owned(),
        }
    }
}

impl<'a> From<&'a ast::Variable> for Variable {
    fn from(value: &ast::Variable) -> Variable {
        let ast::Variable(name) = value;
        Variable::new(None, name)
    }
}
