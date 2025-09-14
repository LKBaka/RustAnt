use crate::ast::ast::{IExpression, INode};
use crate::ast::expr::Expression;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct ReturnExpression {
    pub value: Box<Expression>,
    token: Token,
}

impl INode for ReturnExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl IExpression for ReturnExpression {}

pub fn create_return_expression(token: Token, value: Box<Expression>) -> ReturnExpression {
    ReturnExpression { token, value }
}
