use std::any::Any;

use crate::ast::ast::Statement;
use crate::ast::statements::block_statement::BlockStatement;
use crate::ast::statements::while_statement::create_while_statement;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;
use crate::token::token_type::TokenType::LBrace;

use super::parse_block_statement::parse_block_statement;

pub fn parse_while_statement(parser: &mut Parser) -> Option<Box<dyn Statement>> {
    let token = parser.cur_token.clone();

    parser.next_token(); // 离开 while 词法单元

    // 条件
    let condition = parser.parse_expression(Precedence::Lowest);
    if let Option::None = condition {
        parser.push_err(format!("missing condition",));

        return None;
    }

    if !parser.expect_peek(LBrace) {
        return None;
    }

    parser.next_token(); // 离开条件表达式

    let block = parse_block_statement(parser);
    if block.is_none() {
        parser.push_err(format!("missing while body.",));
        return None;
    }

    Some(Box::new(create_while_statement(
        token,
        condition.unwrap(),
        (block.unwrap() as Box<dyn Any>)
            .downcast_ref::<BlockStatement>()
            .expect("")
            .clone(),
    )))
}
