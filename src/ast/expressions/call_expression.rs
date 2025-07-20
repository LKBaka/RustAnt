use std::any::Any;
use std::ops::Deref;

use crate::ast::ast::{Expression, Node};
use crate::ast::expressions::assignment_expression::AssignmentExpression;
use crate::ast::expressions::class_member_expression::ClassMemberExpression;
use crate::ast::expressions::function_expression::FunctionExpression;
use crate::ast::expressions::identifier::Identifier;
use crate::constants::uninit_obj;
use crate::environment::environment::Environment;
use crate::evaluator::evaluator::Evaluator;
use crate::evaluator::utils::eval_expressions;
use crate::function_caller::function_caller::{call_function, call_native_function, find_function};
use crate::object::ant_function::AntFunction;
use crate::object::ant_native_function::AntNativeFunction;
use crate::object::function_enum::Function;
use crate::object::object::EnvGetter;
use crate::object::utils::{create_error, is_error};
use crate::rc_ref_cell;
use crate::token::token::Token;

use super::super::super::object::object::Object;
use super::object_member_expression::ObjectMemberExpression;

impl Clone for CallExpression {
    fn clone(&self) -> Self {
        Self {
            func: self.func.clone(),
            args: self.args.clone(),
            token: self.token.clone(),
        }
    }
}

pub struct CallExpression {
    pub func: Box<dyn Expression + 'static>,
    pub args: Vec<Box<dyn Expression>>,
    pub token: Token,
}

impl CallExpression {
    fn ident_function_call_handler(
        it: &Identifier, args: &Vec<&Object>, 
        assignment_expressions: Vec<&Box<dyn Expression>>,
        env: &mut Environment, evaluator: &mut Evaluator
    ) -> Option<Object>{
        let func = find_function(it.to_string(), args, env);

        return match func {
            Ok(mut func) => {
                if let Some(it) = (func.clone() as Box<dyn Any>).downcast_ref::<AntFunction>() {
                    // 先与形参环境融合，防止指定参数时参数不在环境中
                    func.get_env_ref().in_place_fusion(&it.param_env);

                    // 将所有参数指定（赋值）表达式进行求值
                    eval_expressions(&assignment_expressions, evaluator, func.get_env_ref());

                    // 清除所有没被设置值的参数
                    func.get_env_ref().remove_obj(&uninit_obj);

                    Some(call_function(Function::Func(func), args, evaluator, env))
                } else if let Some(it) =
                    (func.clone() as Box<dyn Any>).downcast_ref::<AntNativeFunction>()
                {
                    // 先与形参环境融合，防止指定参数时参数不在环境中
                    func.get_env_ref().in_place_fusion(&it.param_env);

                    // 将所有参数指定（赋值）表达式进行求值
                    eval_expressions(&assignment_expressions, evaluator, func.get_env_ref());

                    // 清除所有没被设置值的参数
                    func.get_env_ref().remove_obj(&uninit_obj);

                    call_native_function(Function::NativeFunc(func), args, env)
                } else {
                    None
                }
            }

            Err(it) => Some(it),
        };
    }
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

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Object> {
        // 筛选出所有赋值表达式
        let assignment_expressions = self
            .args
            .iter()
            .filter(|expr| ((*expr) as &dyn Any).is::<AssignmentExpression>())
            .map(|expr| expr)
            .collect::<Vec<&Box<dyn Expression>>>();

        // 筛选出所有不是赋值表达式的表达式，然后求值
        let arg_expressions = self
            .args
            .iter()
            .filter(|expr| !(((*expr) as &dyn Any).is::<AssignmentExpression>()))
            .map(|expr| expr)
            .collect::<Vec<&Box<dyn Expression>>>();

        // 获取拥有所有权的对象列表
        let owned_objects = eval_expressions(&arg_expressions, evaluator, env);
        
        // 创建存储容器（确保生命周期与函数相同）
        let object_store = owned_objects;
        
        // 创建引用列表
        let mut args: Vec<&Object> = object_store.iter().collect();

        let converted_func = self.func.as_mut() as &mut dyn Any;

        // 如果为标识符
        if let Some(it) = converted_func.downcast_ref::<Identifier>() {
            // 尝试调用标识符函数
            return Self::ident_function_call_handler(it, &args, assignment_expressions, env, evaluator);
        } else if let Some(it) = converted_func.downcast_mut::<FunctionExpression>() {
            // 如果为函数表达式

            // 获取函数
            let obj = it.eval(evaluator, env)?;

            let mut func = (obj.as_any().downcast_ref::<AntFunction>()?).clone();
            let func: &mut AntFunction = &mut func;

            // 先与形参环境融合，防止指定参数时参数不在环境中
            func.clone()
                .get_env_ref()
                .in_place_fusion(&func.param_env);

            // 将所有参数指定（赋值）表达式进行求值
            eval_expressions(&assignment_expressions, evaluator, func.get_env_ref());

            // 清除所有没被设置值的参数
            func.get_env_ref().remove_obj(&uninit_obj);

            // 求值
            return Some(call_function(
                Function::Func(Box::new(func.clone())),
                &args,
                evaluator,
                env,
            ));
        } else if let Some(it) = converted_func.downcast_mut::<ObjectMemberExpression>() {
            // 求值出左侧的对象
            let mut left = it.left.eval(evaluator, env)?;
            left.get_env_ref().outer = Some(rc_ref_cell!(env.deref().clone()));

            if is_error(&left) {
                return Some(left);
            }

            // 尝试将右侧表达式转为标识符
            if let Some(ident) = (it.right.as_ref() as &dyn Any).downcast_ref::<Identifier>()
            {
                // 如果右侧表达式为标识符

                let cloned_left = left.clone();

                let mut new_args = vec![&cloned_left];
                new_args.append(&mut args);

                return Self::ident_function_call_handler(
                    ident,
                    &new_args,
                    assignment_expressions,
                    left.get_env_ref(),
                    evaluator,
                )
            }
        } else if let Some(it) = converted_func.downcast_mut::<ClassMemberExpression>() {
            // 求值出左侧的对象
            let mut left = it.left.eval(evaluator, env)?;
            left.get_env_ref().outer = Some(rc_ref_cell!(env.deref().clone()));

            if is_error(&left) {
                return Some(left);
            }

            // 尝试将右侧表达式转为标识符
            if let Some(ident) = (it.right.as_ref() as &dyn Any).downcast_ref::<Identifier>()
            {
                return Self::ident_function_call_handler(
                    ident,
                    &args,
                    assignment_expressions,
                    left.get_env_ref(),
                    evaluator,
                )
            }
        }

        Some(create_error(format!(
            "unsupported function expression: {}",
            self.func.to_string()
        )))
    }
}

impl Expression for CallExpression {}

pub fn create_call_expression(
    token: Token,
    func: Box<dyn Expression>,
    args: Vec<Box<dyn Expression>>,
) -> CallExpression {
    CallExpression { token, func, args }
}
