extern crate clumsy;
use std::collections::HashMap;

#[test]
fn test_factorial_of_3_is_equal_to_6() {
    let source = include_str!("factorial.clumsy");
    println!(
        "{}",
        clumsy::expression::Expression::from_ast_program(
            &clumsy::parser::parse(&mut clumsy::lexer::Lexer::new(source).peekable()).unwrap(),
            &mut HashMap::new(),
        ).evaluate()
    );
}
