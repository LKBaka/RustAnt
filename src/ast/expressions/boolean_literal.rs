use crate::ast::ast::{Expression, Node};

use crate::impl_node;
use crate::token::token::Token;

impl Clone for BooleanLiteral {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            token: self.token.clone(),
        }
    }
}

#[derive(Debug)]
pub struct BooleanLiteral {
    pub value: bool,
    pub token: Token,
}

impl Node for BooleanLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for BooleanLiteral {}

impl_node!(BooleanLiteral);

pub fn create_boolean_literal(token: Token, value: bool) -> BooleanLiteral {
    BooleanLiteral { token, value }
}
