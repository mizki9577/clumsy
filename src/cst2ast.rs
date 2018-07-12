use ast::Expression;
use cst::{Character, ExpressionStatement, Identifier, LetStatement, Number, Program, Statement};

impl<'a> From<&'a Program> for Expression {
    fn from(value: &Program) -> Expression {
        let Program(statements) = value;

        let mut iter = statements.iter();

        let result = if let Some(Statement::Expression(ExpressionStatement { expression })) =
            iter.next_back()
        {
            expression
        } else {
            unimplemented!()
        };

        let mut scopes = iter
            .clone()
            .rev()
            .filter_map(|statement| match statement {
                Statement::Let(LetStatement {
                    variable: Identifier(variable),
                    ..
                }) => Some(variable.as_str()),
                _ => None,
            })
            .collect();

        iter.rfold(
            Expression::from_cst(result, &mut scopes),
            |result, statement| match statement {
                Statement::Expression(..) => unimplemented!(),
                Statement::Let(LetStatement { expression, .. }) => Expression::Application {
                    applicand: box Expression::Abstraction {
                        expression: box result,
                    },
                    argument: box Expression::from_cst(expression, &mut Vec::new()),
                },
            },
        )
    }
}

impl<'a> From<&'a Number> for Expression {
    fn from(value: &Number) -> Expression {
        let Number(value) = value;
        let mut n = value.parse::<usize>().unwrap(); // TODO: handle this
        let mut result = Expression::Variable { index: Some(0) };

        while n > 0 {
            result = Expression::Application {
                applicand: box Expression::Variable { index: Some(1) },
                argument: box result,
            };
            n -= 1;
        }

        Expression::Abstraction {
            expression: box Expression::Abstraction {
                expression: box result,
            },
        }
    }
}

impl<'a> From<&'a Character> for Expression {
    fn from(value: &Character) -> Expression {
        let Character(value) = value;
        let mut n = *value as u32;
        let mut result = Expression::Variable { index: Some(0) };

        while n > 0 {
            result = Expression::Application {
                applicand: box Expression::Variable { index: Some(1) },
                argument: box result,
            };
            n -= 1;
        }

        Expression::Abstraction {
            expression: box Expression::Abstraction {
                expression: box result,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cst::{
        AbstractionExpression, ApplicationExpression, Expression as CSTExpression,
        VariableExpression,
    };

    #[test]
    fn translate_abstraction() {
        let result = Expression::from_cst(
            &CSTExpression::from(AbstractionExpression::new(
                vec![Identifier::new("x"), Identifier::new("x")],
                VariableExpression::new(Identifier::new("x")),
            )),
            &mut Vec::new(),
        );

        let expected = Expression::Abstraction {
            expression: box Expression::Abstraction {
                expression: box Expression::Variable { index: Some(0) },
            },
        };
        assert_eq!(expected, result);

        let b = Expression::from_cst(
            &CSTExpression::from(AbstractionExpression::new(
                vec![Identifier::new("x")],
                ApplicationExpression::new(vec![
                    CSTExpression::from(AbstractionExpression::new(
                        vec![Identifier::new("x")],
                        VariableExpression::new(Identifier::new("x")),
                    )),
                    CSTExpression::from(VariableExpression::new(Identifier::new("x"))),
                ]),
            )),
            &mut Vec::new(),
        );
        let expected = Expression::Abstraction {
            expression: box Expression::Application {
                applicand: box Expression::Abstraction {
                    expression: box Expression::Variable { index: Some(0) },
                },
                argument: box Expression::Variable { index: Some(0) },
            },
        };
        assert_eq!(expected, b);
    }

    #[test]
    fn translate_application() {
        let a = Expression::from_cst(
            &CSTExpression::from(ApplicationExpression::new(vec![
                CSTExpression::from(VariableExpression::new(Identifier::new("a"))),
                CSTExpression::from(VariableExpression::new(Identifier::new("b"))),
                CSTExpression::from(VariableExpression::new(Identifier::new("c"))),
            ])),
            &mut Vec::new(),
        );
        let expected = Expression::Application {
            applicand: box Expression::Application {
                applicand: box Expression::Variable { index: None },
                argument: box Expression::Variable { index: None },
            },
            argument: box Expression::Variable { index: None },
        };
        assert_eq!(expected, a);
    }

    #[test]
    fn translate_let_statement() {
        let expected = Expression::Application {
            applicand: box Expression::Abstraction {
                expression: box Expression::Variable { index: Some(0) },
            },
            argument: box Expression::Abstraction {
                expression: box Expression::Variable { index: Some(0) },
            },
        };
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
