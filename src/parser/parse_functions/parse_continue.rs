use crate::ast::statements::continue_statement::create_continue_statement;
use crate::ast::stmt::Statement;
use crate::parser::parser::Parser;

pub fn parse_continue(parser: &mut Parser) -> Option<Statement> {
    Some(Statement::ContinueStatement(create_continue_statement(
        parser.cur_token.clone(),
    )))
}