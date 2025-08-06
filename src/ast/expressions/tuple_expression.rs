use crate::ast::ast::{Expression, Node};
use crate::constants::null_obj;
use crate::environment::environment::Environment;
use crate::object::object::Object;
use crate::token::token::Token;
use crate::evaluator::evaluator::Evaluator;
use crate::impl_node;

impl Clone for TupleExpression {
    fn clone(&self) -> Self {
        Self {
            expressions: self.expressions.clone(),
            token: self.token.clone()
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
        let expressions_str: Vec<String> = self.expressions.iter()
            .map(|expr| expr.to_string())
            .collect();

        format!("({})", expressions_str.join(", "))
    }

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Object> {
        if self.expressions.len() == 1 {
            self.expressions[0].eval(evaluator, env)
        } else {
            Some(null_obj.clone())
        }
    }
}

impl Expression for TupleExpression {}

impl_node!(TupleExpression);

pub fn create_tuple_expression(token: Token, expressions: Vec<Box<dyn Expression>>) -> TupleExpression {
    TupleExpression {
        token, expressions
    }
}