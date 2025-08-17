use crate::ast::ast::{Expression, Node};

use crate::impl_node;
use crate::token::token::Token;

impl Clone for StringLiteral {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            token: self.token.clone(),
        }
    }
}

#[derive(Debug)]
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

impl_node!(StringLiteral);

pub fn create_string_literal(token: Token, value: String) -> StringLiteral {
    StringLiteral { token, value }
}
