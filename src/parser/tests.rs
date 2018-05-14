use self::ast::*;
use super::*;

#[test]
fn abstraction() {
    let result = parse_abstraction(&mut Lexer::new("\\x y. x").peekable());
    let expected = Ok(AST::Abstraction(ASTAbstraction::new(
        vec![ASTIdentifier::from("x"), ASTIdentifier::from("y")],
        AST::Application(ASTApplication::new(vec![AST::Identifier(
            ASTIdentifier::from("x"),
        )])),
    )));
    assert_eq!(expected, result);
}

#[test]
fn application() {
    let result = parse_application(&mut Lexer::new("x y z").peekable());
    let expected = Ok(AST::Application(ASTApplication::new(vec![
        AST::Identifier(ASTIdentifier::from("x")),
        AST::Identifier(ASTIdentifier::from("y")),
        AST::Identifier(ASTIdentifier::from("z")),
    ])));
    assert_eq!(expected, result);
}
