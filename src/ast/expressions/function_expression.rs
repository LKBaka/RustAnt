use crate::ast::ast::{IExpression, INode};
use crate::ast::expr::Expression;
use crate::ast::expressions::identifier::Identifier;
use crate::ast::statements::block_statement::BlockStatement;
use crate::ast::utils::expressions_to_string;

use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct FunctionExpression {
    pub token: Token,
    pub name: Option<String>,
    pub params: Vec<Box<Expression>>,
    pub return_type: Option<Identifier>,
    pub block: BlockStatement,
}

impl INode for FunctionExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn token(&self) -> Token {
        self.token.clone()
    }

    fn to_string(&self) -> String {
        format!(
            "func {}({}) {{\n{}}}",
            if let Some(it) = &self.name { it } else { "" },
            expressions_to_string(&self.params, ", "),
            self.block.to_string()
        )
    }
}

impl IExpression for FunctionExpression {}

pub fn create_function_expression(
    token: Token,
    name: Option<String>,
    params: Vec<Box<Expression>>,
    block: BlockStatement,
    return_type: Option<Identifier>
) -> FunctionExpression {
    FunctionExpression {
        token,
        name,
        params,
        return_type,
        block,
    }
}
