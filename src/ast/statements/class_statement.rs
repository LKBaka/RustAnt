use uuid::Uuid;

use crate::ast::ast::{Node, Statement};
use crate::ast::expressions::identifier::Identifier;
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::object::ant_class::AntClass;
use crate::object::object::Object;
use crate::evaluator::evaluator::Evaluator;
use crate::object::utils::is_error;
use crate::rc_ref_cell;
use crate::token::token::Token;

use super::block_statement::BlockStatement;

impl Clone for ClassStatement {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            name: self.name.clone(),
            base: self.base.clone(),
            block: self.block.clone()
        }
    }
}

pub struct ClassStatement {
    pub token: Token,
    pub name: Identifier,
    pub base: Option<Identifier>,
    pub block: BlockStatement,
}

impl Node for ClassStatement {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        if let Some(base) = &self.base {
            format!("class {}: {} {{{}}}", self.name.to_string(), base.to_string(), self.block.to_string())
        } else {
            format!("class {} {{{}}}", self.name.to_string(), self.block.to_string())
        }
    }

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Object> {        
        let mut class_env = if let Some(mut base) = self.base.clone() {
            let base_object = base.eval(evaluator, env)?;
            
            base_object.get_env()
        } else {
            Environment::new_with_outer(rc_ref_cell!(env.clone()))
        };

        let block_eval_result = self.block.eval(evaluator, &mut class_env);
        if let Some(it) = block_eval_result {
            if is_error(&it) {
                return Some(it);
            }
        }

        let class_object = Box::new(AntClass {
            id: Uuid::new_v4(),
            base: if let Some(mut base) = self.base.clone() {
                let base_object = base.eval(evaluator, env)?;
                Some(base_object)
            } else {
                None
            },
            env: class_env,
            name: self.name.to_string(),
        });

        let create_result = env.create(
            &self.name.to_string(), Data::new(class_object, DataInfo::new(false))
        );

        match create_result {
            Some(it) => Some(it),
            None => None,
        }
    }
}

impl Statement for ClassStatement {}

pub fn create_class_statement(token: Token, name: Identifier, base: Option<Identifier>, block: BlockStatement) -> ClassStatement {
    ClassStatement {
        token, 
        base,
        name,
        block
    }
}