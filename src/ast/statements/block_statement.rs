use std::ops::Deref;

use crate::ast::ast::{Node, Statement};
use crate::constants::NEW_LINE;

use crate::token::token::Token;
use crate::impl_node;

impl Clone for BlockStatement {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            statements: self.statements.clone(),
        }
    }
}

#[derive(Debug)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Box<dyn Statement>>
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }

    fn to_string(&self) -> String {
        let mut s = String::new();

        for statement in &self.statements {
            s.push_str(format!("{}{}", statement.to_string(), NEW_LINE.to_string()).deref());
        }

        s
    }
}

impl_node!(BlockStatement);

impl Statement for BlockStatement {}

pub fn create_block_statement(token: Token, statements: Vec<Box<dyn Statement>>) -> BlockStatement {
    BlockStatement {
        token,
        statements
    }
}