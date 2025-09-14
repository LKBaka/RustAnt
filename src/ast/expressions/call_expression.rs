use crate::ast::ast::{IExpression, INode};

use crate::ast::expr::Expression;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub func: Box<Expression>,
    pub args: Vec<Box<Expression>>,
    pub token: Token,
}

impl INode for CallExpression {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        let mut args_strings = vec![];

        for arg in self.args.clone() {
            args_strings.push(arg.to_string())
        }

        format!("{}({})", self.func.to_string(), args_strings.join(", "))
    }
}

impl IExpression for CallExpression {}

pub fn create_call_expression(
    token: Token,
    func: Box<Expression>,
    args: Vec<Box<Expression>>,
) -> CallExpression {
    CallExpression { token, func, args }
}
