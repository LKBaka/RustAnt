use crate::token::token::Token;

pub fn print_tokens(tokens: Vec<Token>) {
    for token in tokens {
        println!("{}", token.to_string());
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{token::Token, utils::print_tokens};

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
                TokenType::Integer,
                String::from("1"),
                "__test_print_tokens__".to_string(),
                91,
                91,
            ),
        ]);

        print_tokens(tokens);
    }
}
