use crate::ast::ast::{IExpression, INode};
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct ContinueExpression {
    token: Token,
}

impl INode for ContinueExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn token(&self) -> Token {
        self.token.clone()
    }

    fn to_string(&self) -> String {
        String::from("continue")
    }
}

impl IExpression for ContinueExpression {}

pub fn create_continue_expression(token: Token) -> ContinueExpression {
    ContinueExpression { token }
}
