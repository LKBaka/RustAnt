use crate::ast::ast::{IExpression, INode};

use crate::ast::expr::Expression;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct IndexExpression {
    pub expr: Box<Expression>,
    pub index: Box<Expression>,
    pub token: Token,
}

impl INode for IndexExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("{}[{}]", self.expr.to_string(), self.index.to_string())
    }
}

impl IExpression for IndexExpression {}

pub fn create_index_expression(
    token: Token,
    expr: Box<Expression>,
    index: Box<Expression>,
) -> IndexExpression {
    IndexExpression { token, expr, index }
}
