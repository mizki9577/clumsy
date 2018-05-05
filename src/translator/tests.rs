use self::ast::*;
use super::*;

#[test]
fn translate_abstraction() {
    let a = DeBruijnIndex::from_ast(&Expression::new_abstraction(
        "x",
        Expression::new_variable("x"),
    ));
    let expected = DeBruijnIndex::Abstraction {
        name: "x".to_owned(),
        expression: box DeBruijnIndex::Variable {
            index: Some(0),
            name: "x".to_owned(),
        },
    };
    assert_eq!(expected, a);
}
