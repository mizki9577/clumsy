use super::*;

#[test]
fn abstraction() {
    let result = parse_abstraction(&mut Lexer::new("\\x y. x").peekable());
    let expected = Ok(ASTExpression::Abstraction(ASTAbstraction::new(
        vec![ASTIdentifier::from("x"), ASTIdentifier::from("y")],
        ASTExpression::Application(ASTApplication::new(vec![ASTExpression::Identifier(
            ASTIdentifier::from("x"),
        )])),
    )));
    assert_eq!(expected, result);
}

#[test]
fn application() {
    let result = parse_application(&mut Lexer::new("x y z").peekable());
    let expected = Ok(ASTExpression::Application(ASTApplication::new(vec![
        ASTExpression::Identifier(ASTIdentifier::from("x")),
        ASTExpression::Identifier(ASTIdentifier::from("y")),
        ASTExpression::Identifier(ASTIdentifier::from("z")),
    ])));
    assert_eq!(expected, result);
}
