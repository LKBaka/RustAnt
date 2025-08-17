use crate::token::token_type::TokenType;

use crate::ast::ast::Expression;
use crate::ast::expressions::call_expression::create_call_expression;
use crate::parser::parser::Parser;

pub fn parse_call_expression(
    parser: &mut Parser,
    left: Box<dyn Expression>,
) -> Option<Box<dyn Expression>> {
    let token = parser.cur_token.clone();

    let args = parser.parse_expression_list(TokenType::RParen);

    Some(Box::new(create_call_expression(token, left, args)))
}
