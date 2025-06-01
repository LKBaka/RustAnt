use std::ops::Deref;

use crate::ast::ast::{Expression, Node, Statement};
use crate::ast::expressions::identifier::Identifier;
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::object::object::Object;
use crate::evaluator::evaluator::Evaluator;
use crate::object::utils::is_error;
use crate::token::token::Token;

impl Clone for LetStatement {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            name: self.name.clone(),
            value: self.value.clone()
        }
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.value.clone()
    }

    fn to_string(&self) -> String {
        format!("let {} = {}", self.name.to_string(), self.value.to_string())
    }


    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Object> {
        let value = self.value.eval(evaluator, &mut env.clone()).unwrap();
        
        // 检查值是否为错误对象
        if is_error(&value) {
            // 如果是错误对象，直接返回
            return Some(value);
        }

        let create_result = env.create(
            self.name.to_string().deref(), Data::new(
                value, DataInfo::new(false)
            )
        );

        // 检查返回值是否为空。若为空，代表创建成功
        if create_result.is_some() {
            // 如果返回值不为空，则是创建失败，抛出错误
            Some(create_result.unwrap())
        } else {None}
    }
}

impl Statement for LetStatement {}

pub fn create_let_statement(token: Token, name: Identifier, value: Box<dyn Expression>) -> LetStatement {
    LetStatement {
        token,
        name,
        value
    }
}