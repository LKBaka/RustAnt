use crate::ast::ast::{IExpression, INode};
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct ContinueStatement {
    token: Token,
}

impl INode for ContinueStatement {
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

impl IExpression for ContinueStatement {}

pub fn create_continue_statement(token: Token) -> ContinueStatement {
    ContinueStatement { token }
}
