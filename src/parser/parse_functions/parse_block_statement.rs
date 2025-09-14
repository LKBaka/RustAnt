use crate::ast::stmt::Statement;
use crate::ast::statements::block_statement::create_block_statement;
use crate::parser::parser::Parser;
use crate::token::token_type::TokenType::{self, LBrace, RBrace};

pub fn parse_block_statement(parser: &mut Parser) -> Option<Statement> {
    let token = parser.cur_token.clone();

    if !parser.expect_cur(LBrace) {
        return None;
    }

    parser.next_token(); // 离开左括号

    let mut statements = vec![];


    while !parser.cur_token_is(RBrace) && !parser.cur_token_is(TokenType::Eof) {
        let statement = parser.parse_statement();

        statements.push(statement?);

        parser.next_token() // 离开语句
    }

    // WARNING: 有需要离开右括号的情况自行处理

    Some(Statement::BlockStatement(create_block_statement(token, statements)))
}
