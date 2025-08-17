use crate::ast::ast::{Expression, Node};

use crate::impl_node;
use crate::token::token::Token;

impl Clone for ObjectMemberExpression {
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
            token: self.token.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ObjectMemberExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub token: Token,
}

impl Node for ObjectMemberExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("({}.{})", self.left.to_string(), self.right.to_string())
    }
}

impl Expression for ObjectMemberExpression {}

impl_node!(ObjectMemberExpression);

pub fn create_object_member_expression(
    token: Token,
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
) -> ObjectMemberExpression {
    ObjectMemberExpression { token, left, right }
}
