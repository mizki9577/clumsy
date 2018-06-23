#![feature(underscore_imports)]
#![feature(box_syntax)]
#![feature(box_patterns)]
extern crate clumsy;
use clumsy::expression::{Abstraction, Application, Expression, Variable};
use clumsy::lexer::Lexer;
use clumsy::parser;
use std::io;
use std::io::Read as _;

fn main() {
    let mut source = String::new();
    let _ = io::stdin().read_to_string(&mut source);

    let mut lexer = Lexer::new(&source);
    let ast = parser::parse(&mut lexer).unwrap();
    let expression = Expression::from(&ast);

    println!("{}", unlambda(&encode(expression)));
}

fn encode(value: Expression) -> Expression {
    match value {
        Expression::Variable(..) => value,

        Expression::Abstraction(abstraction) => {
            if !abstraction.is_name_occurs_free() {
                if let Abstraction {
                    expression:
                        box Expression::Application(Application {
                            box callee,
                            argument: box Expression::Variable(Variable { index: Some(0), .. }),
                        }),
                    ..
                } = abstraction
                {
                    return encode(callee.shifted(-1, 0));
                } else {
                    return Expression::Application(Application::new(
                        Expression::Variable(Variable::new(None, "k")),
                        encode((*abstraction.expression).clone().shifted(-1, 0)),
                    ));
                }
            }

            match abstraction {
                Abstraction {
                    expression: box Expression::Variable(Variable { index: Some(0), .. }),
                    ..
                } => Expression::Application(Application::new(
                    Expression::Application(Application::new(
                        Expression::Variable(Variable::new(None, "s")),
                        Expression::Variable(Variable::new(None, "k")),
                    )),
                    Expression::Variable(Variable::new(None, "k")),
                )),

                Abstraction {
                    name: ref x,
                    expression:
                        box Expression::Abstraction(Abstraction {
                            name: ref y,
                            box ref expression,
                        }),
                } => encode(Expression::Abstraction(Abstraction::new(
                    x.as_str(),
                    encode(Expression::Abstraction(Abstraction::new(
                        y.as_str(),
                        expression.clone(),
                    ))),
                ))),

                Abstraction {
                    ref name,
                    expression:
                        box Expression::Application(Application {
                            box ref callee,
                            box ref argument,
                        }),
                } => Expression::Application(Application::new(
                    Expression::Application(Application::new(
                        Expression::Variable(Variable::new(None, "s")),
                        encode(Expression::Abstraction(Abstraction::new(
                            name.as_str(),
                            callee.clone(),
                        ))),
                    )),
                    encode(Expression::Abstraction(Abstraction::new(
                        name.as_str(),
                        argument.clone(),
                    ))),
                )),

                _ => unreachable!(),
            }
        }

        Expression::Application(Application {
            box callee,
            box argument,
        }) => Expression::Application(Application::new(encode(callee), encode(argument))),
    }
}

fn unlambda(value: &Expression) -> String {
    match value {
        Expression::Variable(variable) => format!("{}", variable),
        Expression::Abstraction(..) => panic!(),
        Expression::Application(Application { callee, argument }) => {
            format!("`{}{}", unlambda(callee), unlambda(argument))
        }
    }
}
