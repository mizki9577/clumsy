use super::*;
use lexer::Lexer;

fn new_abstraction(parameter: &str, expression: ast::Expression) -> ast::Expression {
    ast::Expression::Abstraction {
        parameter: ast::Variable::new(parameter),
        expression: box expression,
    }
}

fn new_application(callee: ast::Expression, argument: ast::Expression) -> ast::Expression {
    ast::Expression::Application {
        callee: box callee,
        argument: box argument,
    }
}

fn new_variable(variable: &str) -> ast::Expression {
    ast::Expression::Variable(ast::Variable::new(variable))
}

#[test]
fn parse_abstraction() {
    let a = abstraction(&mut Lexer::new("\\x y. x").peekable());
    let b = abstraction(&mut Lexer::new("\\x. \\y. x").peekable());
    let expected = Ok(new_abstraction(
        "x",
        new_abstraction("y", new_variable("x")),
    ));
    assert_eq!(expected, a);
    assert_eq!(expected, b);
}

#[test]
fn parse_application() {
    let a = application(&mut Lexer::new("x y z").peekable());
    let expected = Ok(new_application(
        new_application(new_variable("x"), new_variable("y")),
        new_variable("z"),
    ));
    assert_eq!(expected, a);
}
