use crate::ast::expr::Expression;
use crate::token::token_type::TokenType;

use crate::ast::expressions::call_expression::create_call_expression;
use crate::parser::parser::Parser;

pub fn parse_call_expression(
    parser: &mut Parser,
    left: Expression,
) -> Option<Expression> {
    let token = parser.cur_token.clone();

    let args = parser.parse_expression_list(TokenType::RParen);

    Some(Expression::CallExpression(create_call_expression(token, Box::new(left), args)))
}
