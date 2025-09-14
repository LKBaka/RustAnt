use crate::ast::ast::{INode, IStatement};
use crate::ast::expressions::identifier::Identifier;

use crate::token::token::Token;

use super::block_statement::BlockStatement;

#[derive(Debug, Clone)]
pub struct ClassStatement {
    pub token: Token,
    pub name: Identifier,
    pub base: Option<Identifier>,
    pub block: BlockStatement,
}

impl INode for ClassStatement {
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

impl IStatement for ClassStatement {}

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
