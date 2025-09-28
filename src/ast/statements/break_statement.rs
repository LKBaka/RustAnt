use crate::ast::ast::{IExpression, INode};
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct BreakStatement {
    token: Token,
}

impl INode for BreakStatement {
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

impl IExpression for BreakStatement {}

pub fn create_break_statement(token: Token) -> BreakStatement {
    BreakStatement { token }
}
