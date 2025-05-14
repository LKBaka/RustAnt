use std::any::Any;
use crate::ast::ast::Expression;
use crate::ast::expressions::function_expression::create_function_expression;
use crate::ast::statements::block_statement::BlockStatement;
use crate::parser::parse_functions::parse_block_statement::parse_block_statement;
use crate::parser::parser::Parser;
use crate::token::token_type::TokenType::{Ident, LParen, RParen};

pub fn parse_function_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
    let mut name: Option<String> = None;
    let token = parser.cur_token.clone();
    
    // 解析参数列表
    parser.next_token(); // 离开 func 关键字

    // 判断是否存在标识符（名字）
    if parser.cur_token_is(Ident) {
        name = Some(parser.cur_token.value.clone());
        parser.next_token(); // 离开标识符
    }

    if !parser.expect_cur(LParen) {
        return None;
    }
    
    parser.next_token(); // 离开左括号

    // 解析参数列表
    let parameters = parser.parse_expression_list(RParen);
    
    if !parser.expect_cur(RParen) {
        return None;
    }
    
    parser.next_token(); // 离开右括号
    
    // 解析函数体
    let body = parse_block_statement(parser);
    if body.is_none() {
        return None;
    }

    let node = body.unwrap() as Box<dyn Any>;

    if let Some(body) = node.downcast_ref::<BlockStatement>() {
        return Some(Box::new(create_function_expression(token, name, parameters, body.to_owned())));
    }

    None
}