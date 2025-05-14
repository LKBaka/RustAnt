use crate::ast::ast::Expression;
use crate::ast::expressions::string_literal::create_string_literal;
use crate::parser::parser::Parser;

pub fn parse_string(parser: &mut Parser) -> Option<Box<dyn Expression>> {
   Some(Box::new(create_string_literal(parser.cur_token.to_owned(), parser.cur_token.value.to_owned())))
}