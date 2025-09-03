use crate::ast::ast::{Expression, Node};
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
    pub token: Token,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for Identifier {}

pub fn create_identifier(token: Token, value: String) -> Identifier {
    Identifier { token, value }
}
