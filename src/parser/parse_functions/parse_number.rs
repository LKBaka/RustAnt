use bigdecimal::BigDecimal;
use std::str::FromStr;

use crate::ast::ast::Expression;
use crate::ast::expressions::double_literal::create_double_literal;
use crate::ast::expressions::integer_literal::create_integer_literal;
use crate::parser::parser::Parser;
use crate::token::token_type::TokenType;

pub fn parse_number(parser: &mut Parser) -> Option<Box<dyn Expression>> {
    let token = parser.cur_token.clone();

    let parse_result = BigDecimal::from_str(&parser.cur_token.value);
    if let Err(_) = parse_result {
        parser.errors.push(format!(
            "could not parse '{}' as integer",
            parser.cur_token.value.clone()
        ));
        return None;
    }

    let value = parse_result.unwrap();

    if !parser.peek_token_is(TokenType::Dot) {
        return Some(Box::new(create_integer_literal(
            token,
            BigDecimal::from(value),
        )));
    }

    parser.next_token(); // 前进至 .(点号)
    parser.next_token();

    let s = &format!("{}.{}", token.value, parser.cur_token.value);
    let parse_result = BigDecimal::from_str(s);

    if let Err(_) = parse_result {
        parser
            .errors
            .push(format!("could not parse '{}' as decimal", s));
        return None;
    }

    let value = parse_result.unwrap();

    return Some(Box::new(create_double_literal(token, value)));
}
