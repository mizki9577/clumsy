use super::*;
use parser::ast::*;

#[test]
fn translate_abstraction() {
    let a = Expression::from(&AST::Abstraction(ASTAbstraction::new(vec!["x", "x"], "x")));
    let expected = Expression::Abstraction(Abstraction::new(
        "x",
        Expression::Abstraction(Abstraction::new(
            "x",
            Expression::Variable(Variable::new(Some(0), "x")),
        )),
    ));
    assert_eq!(expected, a);
}

#[test]
fn translate_application() {
    let a = Expression::from(&AST::Application(ASTApplication::new(vec!["a", "b", "c"])));
    let expected = Expression::Application(Application::new(
        Expression::Application(Application::new(
            Expression::Variable(Variable::new(None, "a")),
            Expression::Variable(Variable::new(None, "b")),
        )),
        Expression::Variable(Variable::new(None, "c")),
    ));
    assert_eq!(expected, a);
}
