use crate::ast::ast::{IExpression, INode};
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct Int64Literal {
    pub value: i64,
    pub token: Token,
}

impl INode for Int64Literal {
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

impl IExpression for Int64Literal {}

pub fn create_int64_literal(token: Token, value: i64) -> Int64Literal {
    Int64Literal { token, value }
}
