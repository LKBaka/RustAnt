use crate::ast::expr::Expression;
use crate::ast::expressions::boolean_literal::create_boolean_literal;
use crate::parser::parser::Parser;
use crate::token::token_type::TokenType::BoolTrue;

pub fn parse_boolean(parser: &mut Parser) -> Option<Expression> {
    Some(Expression::BooleanLiteral(create_boolean_literal(
        parser.cur_token.clone(),
        parser.cur_token_is(BoolTrue),
    )))
}
