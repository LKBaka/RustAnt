use crate::ast::ast::{IExpression, INode};

use crate::ast::expr::Expression;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: Token,
    pub token: Token,
}

impl INode for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn token(&self) -> Token {
        self.token.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.to_string(),
            self.operator.value,
            self.right.to_string()
        )
    }
}

impl IExpression for InfixExpression {}

pub fn create_infix_expression(
    token: Token,
    left: Box<Expression>,
    right: Box<Expression>,
    operator: Token,
) -> InfixExpression {
    InfixExpression {
        token,
        left,
        right,
        operator,
    }
}
