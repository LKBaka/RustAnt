use crate::arg_structure::arg_structure::Args;
use crate::ast::ast::Node;
use crate::environment::environment::Environment;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::object::object::{Object, ERROR};
use crate::evaluator::evaluator::Evaluator;

pub fn eval(code: String, file: String, env: &mut Environment, args: &Args) -> Option<Object> {
    let mut code_lexer: Lexer = Lexer::new(code, file);
    let tokens = code_lexer.get_tokens();

    if code_lexer.contains_error() {
        code_lexer.print_errors();
        return None
    }

    let mut parser: Parser = Parser::new(tokens.clone());
    let mut program = parser.parse_program();

    if parser.contains_error() {
        parser.print_errors();
        return None
    }

    if args.print_ast {
        println!("AST: {}\n", program.to_string());
    }

    let mut evaluator = Evaluator::new();
    let result = evaluator.eval(&mut program, env);

    match result {
        None => {None},
        Some(it) => {
            if it.get_type() != ERROR {
                Some(it)
            } else {
                None
            }
        }
    }
}