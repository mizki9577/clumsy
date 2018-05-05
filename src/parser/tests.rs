use self::ast::*;
use super::*;

#[test]
fn it_can_parse_abstraction() {
    let a = parse_abstraction(&mut Lexer::new("\\x y. x").peekable());
    let b = parse_abstraction(&mut Lexer::new("\\x. \\y. x").peekable());
    let expected = Ok(Expression::new_abstraction(
        "x",
        Expression::new_abstraction("y", Expression::new_variable("x")),
    ));
    assert_eq!(expected, a);
    assert_eq!(expected, b);
}

#[test]
fn it_can_parse_application() {
    let a = parse_application(&mut Lexer::new("x y z").peekable());
    let b = parse_application(&mut Lexer::new("((x) y) z").peekable());
    let expected = Ok(Expression::new_application(
        Expression::new_application(Expression::new_variable("x"), Expression::new_variable("y")),
        Expression::new_variable("z"),
    ));
    assert_eq!(expected, a);
    assert_eq!(expected, b);
}

#[test]
fn it_fails_parsing_abstraction() {
    assert!(parse_abstraction(&mut Lexer::new("x").peekable()).is_err());
    assert!(parse_abstraction(&mut Lexer::new("\\x").peekable()).is_err());
    assert!(parse_abstraction(&mut Lexer::new("\\x (y z). x y z").peekable()).is_err());
}

#[test]
fn it_fails_parsing_application() {
    assert!(parse_application(&mut Lexer::new("(x").peekable()).is_err());
}
