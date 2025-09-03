use bigdecimal::BigDecimal;

use crate::ast::ast::{Expression, Node};
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub value: BigDecimal,
    pub token: Token,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for IntegerLiteral {}

pub fn create_integer_literal(token: Token, value: BigDecimal) -> IntegerLiteral {
    IntegerLiteral { token, value }
}
