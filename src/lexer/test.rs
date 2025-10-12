#[test]
fn test_lexer() {
    use super::lexer::Lexer;

    use crate::token::token::Token;
    use crate::token::token_type::TokenType;
    use crate::utils::assert_eq;

    let file: &'static str = "__test_lexer__";

    let expected_tokens = vec![
        Token::new(TokenType::IntegerBig, "1".into(), file.into(), 1, 1),
        Token::new(TokenType::Eq, "==".into(), file.into(), 1, 3),
        Token::new(TokenType::IntegerBig, "2".into(), file.into(), 1, 6),
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
    use crate::utils::assert_eq;

    let mut l = Lexer::new(
        "let a = 1; // 这是一个注释\nlet b = 2;".to_string(),
        String::from("__test_lexer_comments__"),
    );
    let tokens = l.get_tokens();

    let on_failure_function = || println!("{:#?}", tokens);
    let expected_token_types = vec![
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::IntegerBig,
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::IntegerBig,
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

#[test]
fn test_lexer_string_escapes() {
    use super::lexer::Lexer;
    
    use crate::token::token_type::TokenType;
    use crate::utils::assert_eq;

    let file: &'static str = "__test_lexer_string_escapes__";

    // 测试基本的转义序列
    let code = r#""Hello\nWorld" "Tab\tHere" "Quote\"Inside\"" "Backslash\\" "Carriage\rReturn""#;
    let mut lexer = Lexer::new(code.to_string(), file.into());
    let tokens = lexer.get_tokens();

    let expected_values = vec![
        "Hello\nWorld".to_string(),
        "Tab\tHere".to_string(),
        "Quote\"Inside\"".to_string(),
        "Backslash\\".to_string(),
        "Carriage\rReturn".to_string(),
    ];

    assert_eq(tokens.len(), expected_values.len(), || {
        println!(
            "Expected {} tokens, got {}",
            expected_values.len(),
            tokens.len()
        );
        println!("Tokens: {:?}", tokens);
    });

    for i in 0..tokens.len() {
        assert_eq(tokens[i].token_type, TokenType::String, || {
            println!(
                "Token {} should be String type, got {:?}",
                i, tokens[i].token_type
            );
        });
        assert_eq(&tokens[i].value, &expected_values[i], || {
            println!("Token {} value mismatch", i);
            println!("Expected: {:?}", expected_values[i]);
            println!("Got: {:?}", tokens[i].value);
        });
    }
}

#[test]
fn test_lexer_string_unicode_escapes() {
    use super::lexer::Lexer;
    use crate::token::token_type::TokenType;
    use crate::utils::assert_eq;

    let file: &'static str = "__test_lexer_string_unicode_escapes__";

    // 测试Unicode转义
    let code = r#""\u{0048}\u{0065}\u{006C}\u{006C}\u{006F}" "\u{4F60}\u{597D}""#;
    let mut lexer = Lexer::new(code.to_string(), file.into());
    let tokens = lexer.get_tokens();

    let expected_values = vec![
        "Hello".to_string(),  // \u{0048}\u{0065}\u{006C}\u{006C}\u{006F} -> Hello
        "你好".to_string(),    // \u{4F60}\u{597D} -> 你好
    ];

    assert_eq(tokens.len(), expected_values.len(), || {
        println!("Expected {} tokens, got {}", expected_values.len(), tokens.len());
        println!("Tokens: {:?}", tokens);
    });

    for i in 0..tokens.len() {
        assert_eq(tokens[i].token_type, TokenType::String, || {
            println!("Token {} should be String type, got {:?}", i, tokens[i].token_type);
        });
        assert_eq(&tokens[i].value, &expected_values[i], || {
            println!("Token {} value mismatch", i);
            println!("Expected: {:?}", expected_values[i]);
            println!("Got: {:?}", tokens[i].value);
        });
    }
}

#[test]
fn test_lexer_string_invalid_escapes() {
    use super::lexer::Lexer;

    // 测试无效的转义序列
    let code = r#""Invalid \u escape" "Unclosed string"#;
    let mut lexer = Lexer::new(
        code.to_string(),
        "__test_lexer_string_invalid_escapes__".into(),
    );
    let _tokens = lexer.get_tokens();

    // 应该报告错误
    assert!(
        lexer.contains_error(),
        "Lexer should report error for invalid escape sequences"
    );
}
