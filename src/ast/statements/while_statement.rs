use crate::ast::ast::{INode, IStatement};

use crate::ast::expr::Expression;
use crate::token::token::Token;

use super::block_statement::BlockStatement;

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub token: Token,
    pub condition: Box<Expression>,
    pub block: BlockStatement,
}

impl INode for WhileStatement {
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

impl IStatement for WhileStatement {}

pub fn create_while_statement(
    token: Token,
    condition: Box<Expression>,
    block: BlockStatement,
) -> WhileStatement {
    WhileStatement {
        token,
        condition,
        block,
    }
}
