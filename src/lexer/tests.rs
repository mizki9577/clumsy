use super::*;

#[test]
fn it_accepts_valid_lambda_expression() {
    let result = Lexer::new("((\\x y. x y) (\\z. z)) a").collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            Ok(Token::LeftBracket),
            Ok(Token::LeftBracket),
            Ok(Token::Lambda),
            Ok(Token::Variable("x".to_string())),
            Ok(Token::Variable("y".to_string())),
            Ok(Token::Dot),
            Ok(Token::Variable("x".to_string())),
            Ok(Token::Variable("y".to_string())),
            Ok(Token::RightBracket),
            Ok(Token::LeftBracket),
            Ok(Token::Lambda),
            Ok(Token::Variable("z".to_string())),
            Ok(Token::Dot),
            Ok(Token::Variable("z".to_string())),
            Ok(Token::RightBracket),
            Ok(Token::RightBracket),
            Ok(Token::Variable("a".to_string())),
        ]
    );
}

#[test]
fn it_rejects_invalid_characters() {
    let result = Lexer::new("this = !valid [I think].").collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            Ok(Token::Variable("this".to_string())),
            Err(InvalidCharacter('=')),
            Err(InvalidCharacter('!')),
            Ok(Token::Variable("valid".to_string())),
            Err(InvalidCharacter('[')),
            Ok(Token::Variable("I".to_string())),
            Ok(Token::Variable("think".to_string())),
            Err(InvalidCharacter(']')),
            Ok(Token::Dot),
        ]
    );
}

#[test]
fn it_accepts_and_ignores_very_long_white_spaces() {
    let mut source = String::new();
    for _ in 0..100_000 {
        source.push(' ');
    }
    source.push_str("\\x. x");
    Lexer::new(&source).count();
}
