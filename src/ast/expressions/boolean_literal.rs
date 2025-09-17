use crate::ast::ast::{IExpression, INode};

use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct BooleanLiteral {
    pub value: bool,
    pub token: Token,
}

impl INode for BooleanLiteral {
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

impl IExpression for BooleanLiteral {}

pub fn create_boolean_literal(token: Token, value: bool) -> BooleanLiteral {
    BooleanLiteral { token, value }
}
