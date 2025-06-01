#[test]
fn print_parse_object_member_expression_call() {
    use crate::{ast::ast::Node, lexer::lexer::Lexer, parser::parser::Parser};

    let code = "let a = 1; a.eq(1);";
    let file = "__print_parse_object_member_expression_call__".to_string();

    let mut lexer = Lexer::new(code.into(), file.into());
    let mut parser = Parser::new(lexer.get_tokens());

    let program = parser.parse_program();
    println!("{}", program.to_string());
}