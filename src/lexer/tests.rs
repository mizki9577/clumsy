use super::*;

#[test]
fn lexer_test() {
    assert_eq!(
        Lexer::new("x").next().unwrap(),
        Token::new(TokenType::Variable("x".to_owned()), 0, 0)
    );
    assert_eq!(
        Lexer::new("(").next().unwrap(),
        Token::new(TokenType::LeftBracket, 0, 0)
    );
    assert_eq!(
        Lexer::new(")").next().unwrap(),
        Token::new(TokenType::RightBracket, 0, 0)
    );
    assert_eq!(
        Lexer::new(".").next().unwrap(),
        Token::new(TokenType::Dot, 0, 0)
    );
    assert_eq!(
        Lexer::new("\\").next().unwrap(),
        Token::new(TokenType::Lambda, 0, 0)
    );
    assert_eq!(
        Lexer::new("?").next().unwrap(),
        Token::new(TokenType::InvalidCharacter('?'), 0, 0)
    );
}
