#[test]
fn test_lexer() {
    use super::lexer::Lexer;

    use crate::token::token::Token;
    use crate::token::token_type::TokenType;
    use crate::utils::assert_eq;

    let file: &'static str = "__test_lexer__";

    let expected_tokens = vec![
        Token::new(TokenType::Integer, "1".into(), file.into(), 1, 1),
        Token::new(TokenType::Eq, "==".into(), file.into(), 1, 3),
        Token::new(TokenType::Integer, "2".into(), file.into(), 1, 6),
    ];

    let code = "1 == 2";
    let mut lexer = Lexer::new(code.to_string(), file.into());

    let result = lexer.get_tokens();

    assert_eq(&result, &expected_tokens, || {
        println!("Expected: {:?}", expected_tokens);
        println!("Got: {:?}", result);
    });
}

#[test]
fn test_lexer_2() {
    use super::lexer::Lexer;

    use crate::token::token::Token;
    use crate::token::token_type::TokenType;
    use crate::utils::assert_eq;

    let file: &'static str = "__test_lexer_2__";

    let expected_tokens = vec![
        Token::new(TokenType::Class, "class".into(), file.into(), 1, 1),
        Token::new(TokenType::Ident, "a".into(), file.into(), 1, 7),
        Token::new(TokenType::LBrace, "{".into(), file.into(), 1, 9),
        Token::new(TokenType::RBrace, "}".into(), file.into(), 1, 10),
    ];

    let code = "class a {}";
    let mut lexer = Lexer::new(code.to_string(), file.into());

    let result = lexer.get_tokens();

    assert_eq(&result, &expected_tokens, || {
        println!("Expected: {:?}", expected_tokens);
        println!("Got: {:?}", result);
    });
}

#[test]
fn test_lexer_3() {
    use super::lexer::Lexer;

    use crate::token::token::Token;
    use crate::token::token_type::TokenType;
    use crate::utils::assert_eq;

    let file: &'static str = "__test_lexer_3__";

    let expected_tokens = vec![
        Token::new(TokenType::Ident, "Main".into(), file.into(), 1, 1),
        Token::new(TokenType::GetClassMember, "::".into(), file.into(), 1, 5),
        Token::new(TokenType::Ident, "run".into(), file.into(), 1, 7),
        Token::new(TokenType::LParen, "(".into(), file.into(), 1, 10),
        Token::new(TokenType::RParen, ")".into(), file.into(), 1, 11),
    ];

    let code = "Main::run()";
    let mut lexer = Lexer::new(code.to_string(), file.into());

    let result = lexer.get_tokens();

    assert_eq(&result, &expected_tokens, || {
        println!("Expected: {:?}", expected_tokens);
        println!("Got: {:?}", result);
    });
}

#[test]
fn test_lexer_print_tokens() {
    use super::lexer::Lexer;

    let file: &'static str = "__test_lexer_print_tokens__";

    let code = "// 一个注释\n TestPrint 1;";
    let mut lexer = Lexer::new(code.to_string(), file.into());

    let result = lexer.get_tokens();

    println!("{result:?}")
}

#[test]
fn test_lexer_comments() {
    use super::lexer::Lexer;
    use crate::token::token_type::TokenType;
    use crate::token::utils::print_tokens;
    use crate::utils::assert_eq;

    let mut l = Lexer::new(
        "let a = 1; // 这是一个注释\nlet b = 2;".to_string(),
        String::from("__test_lexer_comments__"),
    );
    let tokens = l.get_tokens();

    let on_failure_function = || print_tokens(tokens.clone());
    let expected_token_types = vec![
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Integer,
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Integer,
        TokenType::Semicolon,
    ];

    // 验证词法单元（注释应该被跳过）
    assert_eq(
        tokens.len(),
        expected_token_types.len(),
        on_failure_function,
    );
    for i in 0..tokens.len() {
        assert_eq(
            tokens[i].token_type,
            expected_token_types[i],
            on_failure_function,
        );
    }
}
