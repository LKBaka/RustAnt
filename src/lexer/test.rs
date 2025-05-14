#[test]
fn test_lexer() {
    use super::lexer::Lexer;
    
    use crate::token::token_type::TokenType;
    use crate::token::token::Token;
    use crate::utils::assert_eq;

    let file: &'static str = "__test_lexer__";


    let expected_tokens = vec![
        Token::new(TokenType::Integer, "1".into(), file.into(), 1),
        Token::new(TokenType::Eq, "==".into(), file.into(), 1),
        Token::new(TokenType::Integer, "2".into(), file.into(), 1),
    ];

    let code = "1 == 2";
    let mut lexer = Lexer::new(code.to_string(), file.into());
    
    let result = lexer.get_tokens();

    assert_eq(result, expected_tokens, || {});
}