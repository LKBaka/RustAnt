use crate::ast::ast::Expression;
use crate::ast::expressions::none_literal::create_none_literal;
use crate::parser::parser::Parser;

pub fn parse_none(parser: &mut Parser) -> Option<Box<dyn Expression>> {
    Some(Box::new(create_none_literal(parser.cur_token.clone())))
}
