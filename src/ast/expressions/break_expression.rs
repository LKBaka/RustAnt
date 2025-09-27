use crate::ast::ast::{IExpression, INode};
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct BreakExpression {
    token: Token,
}

impl INode for BreakExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn token(&self) -> Token {
        self.token.clone()
    }

    fn to_string(&self) -> String {
        String::from("break")
    }
}

impl IExpression for BreakExpression {}

pub fn create_break_expression(token: Token) -> BreakExpression {
    BreakExpression { token }
}
