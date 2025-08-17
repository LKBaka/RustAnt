use crate::ast::ast::{Node, Statement};
use crate::ast::expressions::identifier::Identifier;

use crate::impl_node;
use crate::token::token::Token;

use super::block_statement::BlockStatement;

impl Clone for ClassStatement {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            name: self.name.clone(),
            base: self.base.clone(),
            block: self.block.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ClassStatement {
    pub token: Token,
    pub name: Identifier,
    pub base: Option<Identifier>,
    pub block: BlockStatement,
}

impl Node for ClassStatement {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        if let Some(base) = &self.base {
            format!(
                "class {}: {} {{{}}}",
                self.name.to_string(),
                base.to_string(),
                self.block.to_string()
            )
        } else {
            format!(
                "class {} {{{}}}",
                self.name.to_string(),
                self.block.to_string()
            )
        }
    }
}

impl_node!(ClassStatement);

impl Statement for ClassStatement {}

pub fn create_class_statement(
    token: Token,
    name: Identifier,
    base: Option<Identifier>,
    block: BlockStatement,
) -> ClassStatement {
    ClassStatement {
        token,
        base,
        name,
        block,
    }
}
