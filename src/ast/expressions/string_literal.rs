use crate::ast::ast::{IExpression, INode};

use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub value: String,
    pub token: Token,
}

impl INode for StringLiteral {
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

impl IExpression for StringLiteral {}

pub fn create_string_literal(token: Token, value: String) -> StringLiteral {
    StringLiteral { token, value }
}
