use crate::ast::ast::{IExpression, INode};
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct NoneLiteral {
    pub token: Token,
}

impl INode for NoneLiteral {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn token(&self) -> Token {
        self.token.clone()
    }

    fn to_string(&self) -> String {
        "None".to_string()
    }
}

impl IExpression for NoneLiteral {}

pub fn create_none_literal(token: Token) -> NoneLiteral {
    NoneLiteral { token }
}
