use self::ast::*;
use super::*;

#[test]
fn abstraction() {
    let result = parse_abstraction(&mut Lexer::new("\\x y. x").peekable());
    let expected = Ok(Expression::new_abstraction(
        vec![Variable::from("x"), Variable::from("y")],
        Expression::new_application(vec![Expression::new_variable("x")]),
    ));
    assert_eq!(expected, result);
}

#[test]
fn application() {
    let result = parse_application(&mut Lexer::new("x y z").peekable());
    let expected = Ok(Expression::new_application(vec![
        Expression::new_variable("x"),
        Expression::new_variable("y"),
        Expression::new_variable("z"),
    ]));
    assert_eq!(expected, result);
}
