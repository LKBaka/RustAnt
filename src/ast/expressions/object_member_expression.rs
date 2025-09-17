use crate::ast::ast::{IExpression, INode};

use crate::ast::expr::Expression;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct ObjectMemberExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub token: Token,
}

impl INode for ObjectMemberExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn token(&self) -> Token {
        self.token.clone()
    }

    fn to_string(&self) -> String {
        format!("({}.{})", self.left.to_string(), self.right.to_string())
    }
}

impl IExpression for ObjectMemberExpression {}

pub fn create_object_member_expression(
    token: Token,
    left: Box<Expression>,
    right: Box<Expression>,
) -> ObjectMemberExpression {
    ObjectMemberExpression { token, left, right }
}
