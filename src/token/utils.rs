#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]

use crate::token::token::Token;

pub fn print_tokens(tokens: Vec<Token>) {
    for token in tokens {
        println!("{}", token.to_string());
    }
}

#[test]
fn test_print_tokens() {
    use crate::token::token_type::TokenType;

    let tokens: Vec<Token> = Vec::from([
        Token::new(
            TokenType::Let,
            String::from("let"),
            "__test_print_tokens__".to_string(),
            -91,
        ),
        Token::new(
            TokenType::Ident,
            String::from("a"),
            "__test_print_tokens__".to_string(),
            -91,
        ),
        Token::new(
            TokenType::Assign,
            String::from("="),
            "__test_print_tokens__".to_string(),
            -91,
        ),
        Token::new(
            TokenType::Integer,
            String::from("1"),
            "__test_print_tokens__".to_string(),
            -91,
        ),
    ]);

    print_tokens(tokens);
}
