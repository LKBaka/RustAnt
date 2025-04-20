use crate::ast::ast::Statement;
use crate::ast::statements::block_statement::create_block_statement;
use crate::parser::parser::Parser;
use crate::token::token_type::TokenType::{Eof, RBrace};

pub fn parse_block_statement(parser: &mut Parser) -> Option<Box<dyn Statement>> {
    let token = parser.cur_token.clone();
    let mut statements = vec![];

    parser.next_token(); // 离开左括号

    while !parser.cur_token_is(RBrace) && !parser.cur_token_is(Eof) {
        let statement = parser.parse_statement();
        if let Some(it) = statement {
            statements.push(it);
        }

        parser.next_token(); // 离开语句
    }

    Some(Box::new(create_block_statement(token, statements)))
}