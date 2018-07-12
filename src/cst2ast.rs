use ast::{Abstraction, Application, Expression, Variable};
use cst::{
    AbstractionExpression, ApplicationExpression, Character, Expression as CSTExpression,
    ExpressionStatement, Identifier, LetStatement, Number, Program, Statement, VariableExpression,
};
use std::collections::HashMap;

impl<'a> From<&'a CSTExpression> for Expression {
    fn from(value: &CSTExpression) -> Expression {
        let mut result = match value {
            CSTExpression::Variable(variable) => Expression::Variable(Variable::from(variable)),

            CSTExpression::Abstraction(abstraction) => {
                Expression::Abstraction(Abstraction::from(abstraction))
            }

            CSTExpression::Application(application) => Expression::from(application),

            CSTExpression::Number(number) => Expression::Abstraction(Abstraction::from(number)),

            CSTExpression::Character(character) => {
                Expression::Abstraction(Abstraction::from(character))
            }
        };

        result.assign_indices(&mut HashMap::new());
        result
    }
}

impl<'a> From<&'a Program> for Expression {
    fn from(value: &Program) -> Expression {
        let Program(statements) = value;

        let mut iter = statements.iter();
        match iter.next_back() {
            Some(Statement::Expression(ExpressionStatement { expression: result })) => {
                let mut result = iter.rfold(Expression::from(result), |result, statement| {
                    match statement {
                        Statement::Expression(..) => unimplemented!(),
                        Statement::Let(LetStatement {
                            variable: Identifier(variable),
                            expression,
                        }) => Expression::Application(Application::new(
                            Expression::Abstraction(Abstraction::new(variable.to_owned(), result)),
                            expression,
                        )),
                    }
                });
                result.assign_indices(&mut HashMap::new());
                result
            }

            Some(Statement::Let(..)) => unimplemented!(),
            None => unreachable!(),
        }
    }
}

impl<'a> From<&'a VariableExpression> for Variable {
    fn from(value: &VariableExpression) -> Variable {
        let VariableExpression {
            identifier: Identifier(identifier),
        } = value;
        Variable::new(None, identifier.as_str())
    }
}

impl<'a> From<&'a AbstractionExpression> for Abstraction {
    fn from(value: &AbstractionExpression) -> Abstraction {
        let mut iter = value.parameters.iter();
        let Identifier(parameter) = iter.next_back().unwrap();

        iter.rfold(
            Abstraction::new(parameter.as_str(), &*value.expression),
            |body, Identifier(parameter)| {
                Abstraction::new(parameter.as_str(), Expression::Abstraction(body))
            },
        )
    }
}

impl<'a> From<&'a Number> for Abstraction {
    fn from(value: &Number) -> Abstraction {
        let Number(value) = value;
        let mut n = value.parse::<usize>().unwrap(); // TODO: handle this
        let mut result = Expression::Variable(Variable::new(0, "x"));

        while n > 0 {
            result = Expression::Application(Application::new(
                Expression::Variable(Variable::new(1, "f")),
                result,
            ));
            n -= 1;
        }

        Abstraction::new("f", Expression::Abstraction(Abstraction::new("x", result)))
    }
}

impl<'a> From<&'a Character> for Abstraction {
    fn from(value: &Character) -> Abstraction {
        let Character(value) = value;
        let mut n = *value as u32;
        let mut result = Expression::Variable(Variable::new(0, "x"));

        while n > 0 {
            result = Expression::Application(Application::new(
                Expression::Variable(Variable::new(1, "f")),
                result,
            ));
            n -= 1;
        }

        Abstraction::new("f", Expression::Abstraction(Abstraction::new("x", result)))
    }
}

impl<'a> From<&'a ApplicationExpression> for Expression {
    fn from(value: &ApplicationExpression) -> Expression {
        let mut iter = value.expressions.iter();
        let callee = iter.next().unwrap();

        if let Some(argument) = iter.next() {
            iter.fold(
                Expression::Application(Application::new(callee, argument)),
                |callee, argument| Expression::Application(Application::new(callee, argument)),
            )
        } else {
            Expression::from(callee)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn translate_abstraction() {
        let result = Expression::from(&CSTExpression::from(AbstractionExpression::new(
            vec![Identifier::new("x"), Identifier::new("x")],
            VariableExpression::new(Identifier::new("x")),
        )));

        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Abstraction(Abstraction::new(
                "x",
                Expression::Variable(Variable::new(Some(0), "x")),
            )),
        ));
        assert_eq!(expected, result);

        let b = Expression::from(&CSTExpression::from(AbstractionExpression::new(
            vec![Identifier::new("x")],
            ApplicationExpression::new(vec![
                CSTExpression::from(AbstractionExpression::new(
                    vec![Identifier::new("x")],
                    VariableExpression::new(Identifier::new("x")),
                )),
                CSTExpression::from(VariableExpression::new(Identifier::new("x"))),
            ]),
        )));
        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Application(Application::new(
                Expression::Abstraction(Abstraction::new(
                    "x",
                    Expression::Variable(Variable::new(Some(0), "x")),
                )),
                Expression::Variable(Variable::new(Some(0), "x")),
            )),
        ));
        assert_eq!(expected, b);
    }

    #[test]
    fn translate_application() {
        let a = Expression::from(&CSTExpression::from(ApplicationExpression::new(vec![
            CSTExpression::from(VariableExpression::new(Identifier::new("a"))),
            CSTExpression::from(VariableExpression::new(Identifier::new("b"))),
            CSTExpression::from(VariableExpression::new(Identifier::new("c"))),
        ])));
        let expected = Expression::Application(Application::new(
            Expression::Application(Application::new(
                Expression::Variable(Variable::new(None, "a")),
                Expression::Variable(Variable::new(None, "b")),
            )),
            Expression::Variable(Variable::new(None, "c")),
        ));
        assert_eq!(expected, a);
    }

    #[test]
    fn translate_let_statement() {
        let expected = Expression::Application(Application::new(
            Expression::Abstraction(Abstraction::new(
                "id",
                Expression::Variable(Variable::new(0, "id")),
            )),
            Expression::Abstraction(Abstraction::new(
                "x",
                Expression::Variable(Variable::new(0, "x")),
            )),
        ));
        let result = Expression::from(&Program(vec![
            Statement::from(LetStatement::new(
                Identifier::new("id"),
                CSTExpression::from(AbstractionExpression::new(
                    vec![Identifier::new("x")],
                    CSTExpression::from(ApplicationExpression::new(vec![CSTExpression::from(
                        VariableExpression::new(Identifier::new("x")),
                    )])),
                )),
            )),
            Statement::from(ExpressionStatement::new(CSTExpression::from(
                VariableExpression::new(Identifier::new("id")),
            ))),
        ]));
        assert_eq!(expected, result);
    }
}
