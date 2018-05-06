use self::ast;
use super::*;

#[test]
fn translate_abstraction() {
    let a = Expression::from_ast(&ast::Expression::new_abstraction(
        vec![ast::Variable::from("x"), ast::Variable::from("x")],
        ast::Expression::new_variable("x"),
    ));
    let expected = Expression::Abstraction {
        name: "x".to_owned(),
        expression: box Expression::Abstraction {
            name: "x".to_owned(),
            expression: box Expression::Variable {
                index: Some(0),
                name: "x".to_owned(),
            },
        },
    };
    assert_eq!(expected, a);
}
