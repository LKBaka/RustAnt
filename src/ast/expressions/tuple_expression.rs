use crate::ast::ast::{IExpression, INode};
use crate::ast::expr::Expression;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct TupleExpression {
    pub expressions: Vec<Box<Expression>>,
    pub token: Token,
}

impl INode for TupleExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn token(&self) -> Token {
        self.token.clone()
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

impl IExpression for TupleExpression {}

pub fn create_tuple_expression(
    token: Token,
    expressions: Vec<Box<Expression>>,
) -> TupleExpression {
    TupleExpression { token, expressions }
}
