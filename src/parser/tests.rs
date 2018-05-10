use self::ast::*;
use super::*;

#[test]
fn abstraction() {
    let result = parse_abstraction(&mut Lexer::new("\\x y. x").peekable());
    let expected = Ok(Expression::Abstraction(Abstraction::new(
        vec![Variable::from("x"), Variable::from("y")],
        Expression::Application(Application::new(vec![Expression::Variable(
            Variable::from("x"),
        )])),
    )));
    assert_eq!(expected, result);
}

#[test]
fn application() {
    let result = parse_application(&mut Lexer::new("x y z").peekable());
    let expected = Ok(Expression::Application(Application::new(vec![
        Expression::Variable(Variable::from("x")),
        Expression::Variable(Variable::from("y")),
        Expression::Variable(Variable::from("z")),
    ])));
    assert_eq!(expected, result);
}
