use crate::ast::expr::Expression;
use crate::token::token_type::TokenType;

use crate::ast::expressions::array_literal::create_array_literal;
use crate::parser::parser::Parser;

pub fn parse_array_literal(parser: &mut Parser) -> Option<Expression> {
    let token = parser.cur_token.clone();

    let items = parser.parse_expression_list(TokenType::RBracket);

    Some(Expression::ArrayLiteral(create_array_literal(token, items)))
}
