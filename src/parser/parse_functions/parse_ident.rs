use crate::ast::ast::Expression;
use crate::ast::expressions::identifier::create_identifier;
use crate::parser::parser::Parser;

pub fn parse_ident(parser: &mut Parser) -> Option<Box<dyn Expression>> {
    Some(
        Box::new(
            create_identifier(parser.cur_token.clone(), parser.cur_token.value.clone())
        )
    )
}