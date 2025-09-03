use crate::ast::ast::{Expression, Node};
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct NoneLiteral {
    pub token: Token,
}

impl Node for NoneLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        "None".to_string()
    }
}

impl Expression for NoneLiteral {}

pub fn create_none_literal(token: Token) -> NoneLiteral {
    NoneLiteral { token }
}
