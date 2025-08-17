use crate::ast::ast::{Expression, Node};

use crate::impl_node;
use crate::token::token::Token;

impl Clone for CallExpression {
    fn clone(&self) -> Self {
        Self {
            func: self.func.clone(),
            args: self.args.clone(),
            token: self.token.clone(),
        }
    }
}

#[derive(Debug)]
pub struct CallExpression {
    pub func: Box<dyn Expression + 'static>,
    pub args: Vec<Box<dyn Expression>>,
    pub token: Token,
}

impl Node for CallExpression {
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

impl_node!(CallExpression);

impl Expression for CallExpression {}

pub fn create_call_expression(
    token: Token,
    func: Box<dyn Expression>,
    args: Vec<Box<dyn Expression>>,
) -> CallExpression {
    CallExpression { token, func, args }
}
