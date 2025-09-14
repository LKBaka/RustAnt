use crate::ast::ast::{INode, IStatement};
use crate::ast::expressions::identifier::Identifier;

use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct UseStatement {
    pub token: Token,
    pub name: Identifier,
    pub alias: Option<Identifier>,
}

impl INode for UseStatement {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        match &self.alias {
            Some(alias) => format!("use {} as {}", self.name.to_string(), alias.to_string()),
            None => format!("use {}", self.name.to_string())
        }
    }
}

impl IStatement for UseStatement {}

pub fn create_use_statement(
    token: Token,
    name: Identifier,
    alias: Option<Identifier>,
) -> UseStatement {
    UseStatement { token, name, alias }
}
