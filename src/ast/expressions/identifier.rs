use crate::ast::ast::{IExpression, INode};
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
    pub token: Token,
}

impl INode for Identifier {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl IExpression for Identifier {}

pub fn create_identifier(token: Token, value: String) -> Identifier {
    Identifier { token, value }
}
