use crate::ast::ast::{IExpression, INode};
use crate::ast::expr::Expression;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub operator: String,
    pub expression: Box<Expression>,
    pub token: Token,
}

impl INode for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("{}{}", self.operator, self.expression.to_string())
    }
}

impl IExpression for PrefixExpression {}

pub fn create_prefix_expression(
    token: Token,
    operator: String,
    expression: Box<Expression>,
) -> PrefixExpression {
    PrefixExpression {
        token,
        operator,
        expression,
    }
}
