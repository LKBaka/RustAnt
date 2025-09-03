use crate::ast::ast::{Expression, Node};

use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct IndexExpression {
    pub expr: Box<dyn Expression>,
    pub index: Box<dyn Expression>,
    pub token: Token,
}

impl Node for IndexExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("{}[{}]", self.expr.to_string(), self.index.to_string())
    }
}

impl Expression for IndexExpression {}

pub fn create_index_expression(
    token: Token,
    expr: Box<dyn Expression>,
    index: Box<dyn Expression>,
) -> IndexExpression {
    IndexExpression { token, expr, index }
}
