use interpreter::{Abstraction, Application, Variable};

use parser::ast::{ASTAbstraction, ASTApplication, ASTIdentifier, AST};
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
}

impl<'a> From<&'a AST> for Expression {
    fn from(value: &AST) -> Self {
        let mut result = match value {
            AST::Abstraction(ASTAbstraction {
                parameters,
                box expression,
            }) => {
                let mut iter = parameters.iter();
                let ASTIdentifier(parameter) = iter.next_back().unwrap();
                Expression::Abstraction(iter.rfold(
                    Abstraction::new(parameter, expression.into()),
                    |body, ASTIdentifier(parameter)| {
                        Abstraction::new(parameter, Expression::Abstraction(body))
                    },
                ))
            }
            AST::Application(ASTApplication { expressions }) => {
                let mut iter = expressions.iter();
                let callee = iter.next().unwrap();
                if let Some(argument) = iter.next() {
                    Expression::Application(iter.fold(
                        Application::new(callee.into(), argument.into()),
                        |callee, argument| {
                            Application::new(Expression::Application(callee), argument.into())
                        },
                    ))
                } else {
                    callee.into()
                }
            }
            AST::Identifier(identifier) => Expression::Variable(identifier.into()),
        };
        result.assign_indices();
        result
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expression::Abstraction(abstraction) => abstraction.fmt(f),
            Expression::Application(application) => application.fmt(f),
            Expression::Variable(variable) => variable.fmt(f),
        }
    }
}
