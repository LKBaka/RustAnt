use crate::token::token_type::TokenType;

use crate::ast::ast::Expression;
use crate::ast::expressions::tuple_expression::create_tuple_expression;
use crate::parser::parser::Parser;

pub fn parse_tuple_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
    let token = parser.cur_token.clone();

    let expressions = parser.parse_expression_list(TokenType::RParen);

    Some(Box::new(create_tuple_expression(token, expressions)))
}
