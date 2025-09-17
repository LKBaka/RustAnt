use bigdecimal::BigDecimal;

use crate::ast::ast::{IExpression, INode};
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub value: BigDecimal,
    pub token: Token,
}

impl INode for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn token(&self) -> Token {
        self.token.clone()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl IExpression for IntegerLiteral {}

pub fn create_integer_literal(token: Token, value: BigDecimal) -> IntegerLiteral {
    IntegerLiteral { token, value }
}
