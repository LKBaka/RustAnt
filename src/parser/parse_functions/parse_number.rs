use std::str::FromStr;
use num_bigint::BigInt;

use crate::ast::ast::Expression;
use crate::ast::expressions::integer_literal::create_integer_literal;
use crate::parser::parser::Parser;

pub fn parse_number(parser: &mut Parser) -> Option<Box<dyn Expression>> {
    let mut integer_literal = create_integer_literal(parser.cur_token.clone(), BigInt::ZERO);

    let parse_result = BigInt::from_str(&parser.cur_token.value.clone());
    match parse_result {
        Err(_) => {
            parser.errors.push(format!("could not parse \"{}\" as integer", parser.cur_token.value.clone()));
        }
        Ok(it) => {
            integer_literal.value = it;
        }
    }

    Some(Box::new(integer_literal))
}