extern crate clumsy;

#[test]
fn test_factorial_of_3_is_equal_to_6() {
    let source = include_str!("factorial.clumsy");
    println!(
        "{}",
        clumsy::expression::Expression::from(
            &clumsy::parser::parse(&mut clumsy::lexer::Lexer::new(source)).unwrap()
        ).evaluate()
    );
}
