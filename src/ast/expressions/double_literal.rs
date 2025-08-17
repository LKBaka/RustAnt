use bigdecimal::BigDecimal;

use crate::ast::ast::{Expression, Node};

use crate::impl_node;
use crate::token::token::Token;

impl Clone for DoubleLiteral {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            token: self.token.clone(),
        }
    }
}

#[derive(Debug)]
pub struct DoubleLiteral {
    pub value: BigDecimal,
    pub token: Token,
}

impl Node for DoubleLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for DoubleLiteral {}

impl_node!(DoubleLiteral);

pub fn create_double_literal(token: Token, value: BigDecimal) -> DoubleLiteral {
    DoubleLiteral { token, value }
}
