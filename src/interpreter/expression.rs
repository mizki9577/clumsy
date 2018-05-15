use interpreter::{Abstraction, Application, Variable};

use parser::ast::{ASTAbstraction, ASTApplication, ASTIdentifier, AST};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Abstraction(Abstraction),
    Application(Application),
    Variable(Variable),
}

impl Expression {
    // Is this should be immutable?
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

    fn shift(&mut self, d: usize, c: usize) {
        match self {
            Expression::Abstraction(Abstraction { expression, .. }) => {
                expression.shift(d, c + 1);
            }
            Expression::Application(Application { callee, argument }) => {
                callee.shift(d, c);
                argument.shift(d, c);
            }
            Expression::Variable(Variable {
                index: Some(index), ..
            }) if *index >= c =>
            {
                *index += d
            }
            _ => (),
        }
    }

    fn substitute(&mut self, j: usize, mut term: Expression) {
        match self {
            Expression::Abstraction(Abstraction { expression, .. }) => {
                term.shift(1, 0);
                expression.substitute(j + 1, term);
            }
            Expression::Application(Application { callee, argument }) => {
                let cloned_term = term.clone();
                callee.substitute(j, term);
                argument.substitute(j, cloned_term);
            }
            Expression::Variable(Variable {
                index: Some(index), ..
            }) if *index == j =>
            {
                *self = term;
            }
            _ => (),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shift() {
        let mut variable = Expression::Variable(Variable::new(Some(0), "x"));
        let expected = Expression::Variable(Variable::new(Some(1), "x"));
        variable.shift(1, 0);
        assert_eq!(expected, variable);

        let mut variable = Expression::Variable(Variable::new(Some(0), "x"));
        let expected = Expression::Variable(Variable::new(Some(0), "x"));
        variable.shift(1, 1);
        assert_eq!(expected, variable);

        let mut free_abstraction = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(1), "y")),
        ));
        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(2), "y")),
        ));
        free_abstraction.shift(1, 0);
        assert_eq!(expected, free_abstraction);

        let mut bound_abstraction = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(0), "x")),
        ));
        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(0), "x")),
        ));
        bound_abstraction.shift(1, 0);
        assert_eq!(expected, bound_abstraction);

        let mut application = Expression::Application(Application::new(
            Expression::Variable(Variable::new(Some(0), "x")),
            Expression::Variable(Variable::new(Some(1), "y")),
        ));
        let expected = Expression::Application(Application::new(
            Expression::Variable(Variable::new(Some(1), "x")),
            Expression::Variable(Variable::new(Some(2), "y")),
        ));
        application.shift(1, 0);
        assert_eq!(expected, application);
    }

    #[test]
    fn test_substitute() {
        let mut variable = Expression::Variable(Variable::new(Some(0), "x"));
        let expected = Expression::Variable(Variable::new(None, "a"));
        variable.substitute(0, Expression::Variable(Variable::new(None, "a")));
        assert_eq!(expected, variable);

        let mut variable = Expression::Variable(Variable::new(Some(1), "x"));
        let expected = Expression::Variable(Variable::new(Some(1), "x"));
        variable.substitute(0, Expression::Variable(Variable::new(None, "a")));
        assert_eq!(expected, variable);

        let mut free_abstraction = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(1), "y")),
        ));
        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(None, "a")),
        ));
        free_abstraction.substitute(0, Expression::Variable(Variable::new(None, "a")));
        assert_eq!(expected, free_abstraction);

        let mut bound_abstraction = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(0), "x")),
        ));
        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(0), "x")),
        ));
        bound_abstraction.substitute(0, Expression::Variable(Variable::new(None, "a")));
        assert_eq!(expected, bound_abstraction);

        let mut application = Expression::Application(Application::new(
            Expression::Variable(Variable::new(Some(0), "x")),
            Expression::Variable(Variable::new(Some(1), "y")),
        ));
        let expected = Expression::Application(Application::new(
            Expression::Variable(Variable::new(Some(0), "x")),
            Expression::Variable(Variable::new(None, "a")),
        ));
        application.substitute(1, Expression::Variable(Variable::new(None, "a")));
        assert_eq!(expected, application);
    }
}
