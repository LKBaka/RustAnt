use crate::ast::ast::{INode, IStatement};
use crate::ast::expr::Expression;
use crate::ast::expressions::identifier::Identifier;

use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<Expression>,
}

impl INode for LetStatement {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn token(&self) -> Token {
        self.token.clone()
    }

    fn to_string(&self) -> String {
        format!("let {} = {}", self.name.to_string(), self.value.to_string())
    }
}

impl IStatement for LetStatement {}

pub fn create_let_statement(
    token: Token,
    name: Identifier,
    value: Box<Expression>,
) -> LetStatement {
    LetStatement { token, name, value }
}
