use crate::ast::ast::{Expression, Node, Statement};
use crate::environment::environment::Environment;
use crate::object::object::IAntObject;
use crate::evaluator::evaluator::Evaluator;
use crate::object::utils::is_truthy;
use crate::token::token::Token;

use super::block_statement::BlockStatement;

impl Clone for WhileStatement {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            condition: self.condition.clone(),
            block: self.block.clone()
        }
    }
}

pub struct WhileStatement {
    pub token: Token,
    pub condition: Box<dyn Expression>,
    pub block: BlockStatement,
}

impl Node for WhileStatement {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("while {} {{{}}}", self.condition.to_string(), self.block.to_string())
    }

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Box<dyn IAntObject>> {        
        loop {
            if let Some(it) = self.condition.eval(evaluator, env) {
                if !is_truthy(it) {break;}
            }

            self.block.eval(evaluator, env);
        }

        None
    }
}

impl Statement for WhileStatement {}

pub fn create_while_statement(token: Token, condition: Box<dyn Expression>, block: BlockStatement) -> WhileStatement {
    WhileStatement {
        token,
        condition,
        block
    }
}