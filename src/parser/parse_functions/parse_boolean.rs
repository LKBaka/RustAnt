use crate::ast::ast::Expression;
use crate::ast::expressions::boolean_literal::create_boolean_literal;
use crate::parser::parser::Parser;
use crate::token::token_type::TokenType::BoolTrue;

pub fn parse_boolean(parser: &mut Parser) -> Option<Box<dyn Expression>> {
    Some(
        Box::new(
            create_boolean_literal(parser.cur_token.to_owned(), parser.cur_token_is(BoolTrue))
        )
    )
}