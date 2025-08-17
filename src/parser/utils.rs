use crate::ast::ast::Program;

pub fn parse(code: String, file: String) -> Result<Program, ()> {
    use crate::{lexer::lexer::Lexer, parser::parser::Parser};

    let mut code_lexer: Lexer = Lexer::new(code, file);
    let tokens = code_lexer.get_tokens();

    if code_lexer.contains_error() {
        code_lexer.print_errors();
        return Err(());
    }

    let mut parser: Parser = Parser::new(tokens.clone());
    let program = parser.parse_program();

    if parser.contains_error() {
        parser.print_errors();
        return Err(());
    }

    Ok(program)
}
