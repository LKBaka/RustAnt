use crate::ast::ast::{Expression, Node};
use crate::token::token::Token;

impl Clone for ReturnExpression {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            token: self.token.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ReturnExpression {
    pub value: Box<dyn Expression>,
    token: Token,
}

impl Node for ReturnExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for ReturnExpression {}

pub fn create_return_expression(token: Token, value: Box<dyn Expression>) -> ReturnExpression {
    ReturnExpression { token, value }
}
