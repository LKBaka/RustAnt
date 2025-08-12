
use crate::ast::ast::{Expression, Node};
use crate::ast::statements::block_statement::BlockStatement;
use crate::ast::utils::expressions_to_string;

use crate::token::token::Token;
use crate::impl_node;

impl Clone for FunctionExpression {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            name: self.name.clone(),
            params: self.params.clone(),
            block: self.block.clone(),
        }
    }
}

#[derive(Debug)]
pub struct FunctionExpression {
    pub token: Token,
    pub name: Option<String>,
    pub params: Vec<Box<dyn Expression>>,
    pub block: BlockStatement,
}

impl Node for FunctionExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "func {}({}) {{{}}}",
            if let Some(it) = &self.name {
                it
            } else { "" },
            expressions_to_string(&self.params, ", "),
            self.block.to_string()
        )
    }
}

impl_node!(FunctionExpression);

impl Expression for FunctionExpression {}

pub fn create_function_expression(token: Token, name: Option<String>, params: Vec<Box<dyn Expression>>, block: BlockStatement) -> FunctionExpression {
    FunctionExpression {
        token,
        name,
        params,
        block
    }
}