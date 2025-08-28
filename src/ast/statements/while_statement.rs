use crate::ast::ast::{Expression, Node, Statement};

use crate::token::token::Token;

use super::block_statement::BlockStatement;

impl Clone for WhileStatement {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            condition: self.condition.clone(),
            block: self.block.clone(),
        }
    }
}

#[derive(Debug)]
pub struct WhileStatement {
    pub token: Token,
    pub condition: Box<dyn Expression>,
    pub block: BlockStatement,
}

impl Node for WhileStatement {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "while {} {{{}}}",
            self.condition.to_string(),
            self.block.to_string()
        )
    }
}

impl Statement for WhileStatement {}

pub fn create_while_statement(
    token: Token,
    condition: Box<dyn Expression>,
    block: BlockStatement,
) -> WhileStatement {
    WhileStatement {
        token,
        condition,
        block,
    }
}
