use crate::ast::ast::Expression;
use crate::ast::expressions::hash_literal::create_hash_literal;
use crate::parser::parser::Parser;
use crate::parser::precedence::Precedence;
use crate::token::token_type::TokenType;

pub fn parse_hash_literal(parser: &mut Parser) -> Option<Box<dyn Expression>> {
    let token = parser.cur_token.clone();

    let mut pairs = vec![];

    while !parser.peek_token_is(TokenType::RBrace) {
        parser.next_token();

        let key = parser.parse_expression(Precedence::Lowest)?;

        if !parser.expect_peek(TokenType::Colon) {
            return None
        }

        parser.next_token();
        parser.next_token();

        let value = parser.parse_expression(Precedence::Lowest)?;
        pairs.push((key, value));

        if !parser.peek_token_is(TokenType::RBrace) && !parser.expect_peek(TokenType::Comma) {
            return None
        }

        if parser.peek_token_is(TokenType::Comma) {
            parser.next_token();
        }
    }

    if !parser.expect_peek(TokenType::RBrace) {
        return None
    }

    parser.next_token();

    Some(Box::new(create_hash_literal(token, pairs)))
}
