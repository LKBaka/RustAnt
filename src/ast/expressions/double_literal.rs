use bigdecimal::BigDecimal;

use crate::ast::ast::{IExpression, INode};

use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct DoubleLiteral {
    pub value: BigDecimal,
    pub token: Token,
}

impl INode for DoubleLiteral {
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

impl IExpression for DoubleLiteral {}

pub fn create_double_literal(token: Token, value: BigDecimal) -> DoubleLiteral {
    DoubleLiteral { token, value }
}
