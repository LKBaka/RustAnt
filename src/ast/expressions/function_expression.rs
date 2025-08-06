use std::any::Any;
use uuid::Uuid;

use crate::ast::ast::{Expression, Node};
use crate::ast::expressions::assignment_expression::AssignmentExpression;
use crate::ast::expressions::identifier::Identifier;
use crate::ast::statements::block_statement::BlockStatement;
use crate::ast::utils::expressions_to_string;
use crate::constants::uninit_obj;
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::object::ant_function::AntFunction;
use crate::object::object::Object;
use crate::evaluator::evaluator::Evaluator;
use crate::rc_ref_cell;
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
    params: Vec<Box<dyn Expression>>,
    block: BlockStatement,
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

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Object> {
        // 筛选出所有赋值表达式
        let assignment_expressions = self.params
            .iter()
            .filter(|expr| {((*expr).as_ref() as &dyn Any).downcast_ref::<AssignmentExpression>().is_some()})
            .map(|expr| expr.clone())
            .collect::<Vec<Box<dyn Expression>>>();

        // 筛选出所有标识符
        let ident_expressions = self.params
            .iter()
            .filter(|expr| {((*expr).as_ref() as &dyn Any).downcast_ref::<Identifier>().is_some()})
            .map(|expr| expr.clone())
            .collect::<Vec<Box<dyn Expression>>>();
        
        // 函数的环境
        let func_env = Environment::new_with_outer(rc_ref_cell!(env.clone()));

        // 函数形参的环境
        let mut param_env = Environment::new_with_outer(rc_ref_cell!(env.clone()));

        // 在形参环境中注册所有标识符
        for ident_expression in ident_expressions {
            let node = ident_expression as Box<dyn Any>;
            let ident_expression = node
                .downcast_ref::<Identifier>()
                .expect("not identifier");

            param_env.create(&ident_expression.to_string(), Data::new(uninit_obj.clone(), DataInfo::new(false)));
        }

        for assignment_expression in assignment_expressions {
            let assignment_expression = assignment_expression as Box<dyn Any>;
            let assignment_expression = assignment_expression
                .downcast_ref::<AssignmentExpression>()
                .expect("not assignment expression");

            let ident = assignment_expression.left.as_ref() as &dyn Any;
            let ident = ident
                .downcast_ref::<Identifier>()
                .expect(&format!("non assignable expression '{}'", &assignment_expression.left.clone().to_string()));

            if let Some(it) = assignment_expression.value.clone().eval(evaluator, env) {
                param_env.create(&ident.to_string(), Data::new(it, DataInfo::new(false)));
            }
        }

        let func = Box::new(
            AntFunction {
                id: Uuid::new_v4(),
                env: func_env,
                param_env,
                block: self.block.clone()
            }
        );

        if let Some(name) = &self.name {
            env.create(name, Data::new(func.clone(), DataInfo::new(false)));
        }

        Some(func)
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