use super::*;
use parser::ast::*;

#[test]
fn translate_abstraction() {
    let a = Expression::from(&AST::Abstraction(ASTAbstraction::new(
        vec![ASTIdentifier::from("x"), ASTIdentifier::from("x")],
        AST::Identifier(ASTIdentifier::from("x")),
    )));
    let expected = Expression::from(Abstraction::new(
        "x",
        Abstraction::new("x", Variable::new(Some(0), "x").into()).into(),
    ));
    assert_eq!(expected, a);
}
