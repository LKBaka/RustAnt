use crate::ast::ast::{Expression, Node};
use crate::token::token::Token;

use crate::impl_node;

impl Clone for PrefixExpression {
    fn clone(&self) -> Self {
        Self {
            operator: self.operator.clone(),
            expression: self.expression.clone(),
            token: self.token.clone()
        }
    }
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub operator: String,
    pub expression: Box<dyn Expression>,
    pub token: Token,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("{}{}", self.operator, self.expression.to_string())
    }
}

impl Expression for PrefixExpression {}

impl_node!(PrefixExpression);

pub fn create_prefix_expression(token: Token, operator: String, expression: Box<dyn Expression>) -> PrefixExpression {
    PrefixExpression {
        token, operator, expression
    }
}