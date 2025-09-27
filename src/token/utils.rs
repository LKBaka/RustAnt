#[cfg(test)]
mod tests {
    use crate::token::token::Token;

    #[test]
    fn test_print_tokens() {
        use crate::token::token_type::TokenType;

        let tokens: Vec<Token> = Vec::from([
            Token::new(
                TokenType::Let,
                String::from("let"),
                "__test_print_tokens__".to_string(),
                91,
                91,
            ),
            Token::new(
                TokenType::Ident,
                String::from("a"),
                "__test_print_tokens__".to_string(),
                91,
                91,
            ),
            Token::new(
                TokenType::Assign,
                String::from("="),
                "__test_print_tokens__".to_string(),
                91,
                91,
            ),
            Token::new(
                TokenType::IntegerBig,
                String::from("1"),
                "__test_print_tokens__".to_string(),
                91,
                91,
            ),
        ]);

        println!("{:#?}", tokens)
    }
}
