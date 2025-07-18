#[test]
fn print_parse_object_member_expression_call() {
    use crate::{ast::ast::Node, lexer::lexer::Lexer, parser::parser::Parser};

    let code = "let a = 1; a.plus(1).plus(1);";
    let file = "__print_parse_object_member_expression_call__".to_string();

    let mut lexer = Lexer::new(code.into(), file.into());
    let mut parser = Parser::new(lexer.get_tokens());

    let program = parser.parse_program();
    println!("{}", program.to_string());
}

#[test]
fn print_ast() {
    use crate::{ast::ast::Node, lexer::lexer::Lexer, parser::parser::Parser};

    let code = "fib(n - 1) + fib(n - 2)";
    let file = "__print_ast__".to_string();

    let mut lexer = Lexer::new(code.into(), file.into());
    let mut parser = Parser::new(lexer.get_tokens());

    let program = parser.parse_program();
    println!("{}", program.to_string());
}

#[test]
fn test_statement_termination() {
    use crate::{ast::ast::Node, lexer::lexer::Lexer, parser::parser::Parser};

    // 测试分号结束的语句
    let code_with_semicolon = "let a = 1; let b = 2;";
    let file = "__test_semicolon__".to_string();

    let mut lexer = Lexer::new(code_with_semicolon.into(), file.into());
    let mut parser = Parser::new(lexer.get_tokens());

    let program = parser.parse_program();
    println!("With semicolon: {}", program.to_string());
    assert!(!parser.contains_error(), "Parser should not have errors with semicolon");

    // 测试换行符结束的语句
    let code_with_newline = "let a = 1\nlet b = 2\n";
    let file2 = "__test_newline__".to_string();

    let mut lexer2 = Lexer::new(code_with_newline.into(), file2.into());
    let mut parser2 = Parser::new(lexer2.get_tokens());

    let program2 = parser2.parse_program();
    println!("With newline: {}", program2.to_string());
    assert!(!parser2.contains_error(), "Parser should not have errors with newline");

    // 测试混合情况
    let code_mixed = "let a = 1\nlet b = 2; let c = 3\n";
    let file3 = "__test_mixed__".to_string();

    let mut lexer3 = Lexer::new(code_mixed.into(), file3.into());
    let mut parser3 = Parser::new(lexer3.get_tokens());

    let program3 = parser3.parse_program();
    println!("Mixed: {}", program3.to_string());
    assert!(!parser3.contains_error(), "Parser should not have errors with mixed termination");
}