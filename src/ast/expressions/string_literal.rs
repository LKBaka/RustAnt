use crate::ast::ast::{Expression, Node};

use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub value: String,
    pub token: Token,
}

impl Node for StringLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for StringLiteral {}

pub fn create_string_literal(token: Token, value: String) -> StringLiteral {
    StringLiteral { token, value }
}
