use crate::ast::ast::{Expression, Node, Statement};
use crate::ast::expressions::identifier::Identifier;

use crate::token::token::Token;

impl Clone for LetStatement {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            name: self.name.clone(),
            value: self.value.clone(),
        }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("let {} = {}", self.name.to_string(), self.value.to_string())
    }
}

impl Statement for LetStatement {}

pub fn create_let_statement(
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
) -> LetStatement {
    LetStatement { token, name, value }
}
