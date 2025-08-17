use crate::ast::ast::{Expression, Node};
use crate::impl_node;
use crate::token::token::Token;

impl Clone for TupleExpression {
    fn clone(&self) -> Self {
        Self {
            expressions: self.expressions.clone(),
            token: self.token.clone(),
        }
    }
}

#[derive(Debug)]
pub struct TupleExpression {
    pub expressions: Vec<Box<dyn Expression>>,
    pub token: Token,
}

impl Node for TupleExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        let expressions_str: Vec<String> = self
            .expressions
            .iter()
            .map(|expr| expr.to_string())
            .collect();

        format!("({})", expressions_str.join(", "))
    }
}

impl Expression for TupleExpression {}

impl_node!(TupleExpression);

pub fn create_tuple_expression(
    token: Token,
    expressions: Vec<Box<dyn Expression>>,
) -> TupleExpression {
    TupleExpression { token, expressions }
}
