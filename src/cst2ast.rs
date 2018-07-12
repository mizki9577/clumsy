use ast::{Abstraction, Application, Expression, Variable};
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
                Statement::Let(LetStatement {
                    variable: Identifier(variable),
                    expression,
                }) => Expression::Application(Application::new(
                    Expression::Abstraction(Abstraction::new(variable.to_owned(), result)),
                    Expression::from_cst(expression, &mut Vec::new()),
                )),
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

        let expected = Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Abstraction(Abstraction::new(
                "x",
                Expression::Variable(Variable::new(Some(0), "x")),
            )),
        ));
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
        let a = Expression::from_cst(
            &CSTExpression::from(ApplicationExpression::new(vec![
                CSTExpression::from(VariableExpression::new(Identifier::new("a"))),
                CSTExpression::from(VariableExpression::new(Identifier::new("b"))),
                CSTExpression::from(VariableExpression::new(Identifier::new("c"))),
            ])),
            &mut Vec::new(),
        );
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
