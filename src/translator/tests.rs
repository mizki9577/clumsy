use self::ast;
use super::*;

#[test]
fn translate_abstraction() {
    let a = Expression::from(&ast::Expression::Abstraction(ast::Abstraction::new(
        vec![ast::Variable::from("x"), ast::Variable::from("x")],
        ast::Expression::Variable(ast::Variable::from("x")),
    )));
    let expected = Expression::from(Abstraction::new(
        "x",
        Abstraction::new("x", Variable::new(Some(0), "x").into()).into(),
    ));
    assert_eq!(expected, a);
}
