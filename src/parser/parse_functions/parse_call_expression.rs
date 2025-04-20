use crate::ast::ast::Expression;
use crate::ast::expressions::call_expression::create_call_expression;
use crate::parser::parser::Parser;
use crate::token::token_type::TokenType::RParen;

pub fn parse_call_expression(parser: &mut Parser, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
    let token = parser.cur_token.clone();

    let func = left;

    // 离开表达式
    parser.next_token();

    // 解析参数
    let args = parser.parse_expression_list(RParen);

    Some(Box::new(create_call_expression(token, func, args)))
}