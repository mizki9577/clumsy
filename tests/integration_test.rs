extern crate clumsy;

#[test]
fn test_eager() {
    let source = include_str!("factorial.clumsy");
    println!(
        "{}",
        clumsy::interpreter::Expression::from(
            &clumsy::parser::parse(&mut clumsy::lexer::Lexer::new(source).peekable()).unwrap()
        ).evaluate_eager()
    );
}
