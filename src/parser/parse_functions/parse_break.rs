use crate::ast::statements::break_statement::create_break_statement;
use crate::ast::stmt::Statement;
use crate::parser::parser::Parser;

pub fn parse_break(parser: &mut Parser) -> Option<Statement> {
    Some(Statement::BreakStatement(create_break_statement(
        parser.cur_token.clone(),
    )))
}