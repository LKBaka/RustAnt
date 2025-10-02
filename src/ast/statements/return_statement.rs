use crate::ast::ast::{IExpression, INode};
use crate::ast::expr::Expression;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Box<Expression>,
    token: Token,
}

impl INode for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn token(&self) -> Token {
        self.token.clone()
    }

    fn to_string(&self) -> String {
        format!("return {}", self.value.to_string())
    }
}

impl IExpression for ReturnStatement {}

pub fn create_return_statement(token: Token, value: Box<Expression>) -> ReturnStatement {
    ReturnStatement { token, value }
}
