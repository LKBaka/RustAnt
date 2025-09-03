use std::ops::Deref;

use crate::ast::ast::{Node, Statement};
use crate::constants::NEW_LINE;

use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        let mut s = String::new();

        for statement in &self.statements {
            s.push_str(format!("{}{}", statement.to_string(), NEW_LINE.to_string()).deref());
        }

        s
    }
}

impl Statement for BlockStatement {}

pub fn create_block_statement(token: Token, statements: Vec<Box<dyn Statement>>) -> BlockStatement {
    BlockStatement { token, statements }
}
