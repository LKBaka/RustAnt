use std::any::Any;

use crate::evaluator::utils::eval_expressions;
use crate::ast::ast::{Expression, Node};
use crate::ast::expressions::assignment_expression::AssignmentExpression;
use crate::ast::expressions::function_expression::FunctionExpression;
use crate::ast::expressions::identifier::Identifier;
use crate::constants::uninit_obj;
use crate::environment::environment::Environment;
use crate::function_caller::function_caller::{call_function, call_native_function, find_function};
use crate::object::ant_function::AntFunction;
use crate::object::ant_native_function::AntNativeFunction;
use crate::object::object::{GetEnv, IAntObject};
use crate::object::utils::create_error;
use crate::token::token::Token;
use crate::evaluator::evaluator::Evaluator;

impl Clone for CallExpression {
    fn clone(&self) -> Self {
        Self {
            func: self.func.clone(),
            args: self.args.clone(),
            token: self.token.clone()
        }
    }
}

pub struct CallExpression {
    func: Box<dyn Expression + 'static>,
    args: Vec<Box<dyn Expression>>,
    token: Token,
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

    fn eval(&mut self, evaluator: &mut Evaluator, env: &mut Environment) -> Option<Box<dyn IAntObject>> {        // 筛选出所有赋值表达式
        let assignment_expressions = self.args
            .iter()
            .filter(|expr| {((*expr).clone() as Box<dyn Any>).downcast_ref::<AssignmentExpression>().is_some()})
            .map(|expr| expr.clone())
            .collect::<Vec<Box<dyn Expression>>>();

        // 筛选出所有不是赋值表达式的表达式，然后求值
        let arg_expressions = self.args
            .iter()
            .filter(|expr| {((*expr).clone() as Box<dyn Any>).downcast_ref::<AssignmentExpression>().is_none()})
            .map(|expr| expr.clone())
            .collect::<Vec<Box<dyn Expression>>>();

        let args = eval_expressions(&arg_expressions, evaluator, env);

        let converted_box = self.func.clone() as Box<dyn Any>;
        let ident = converted_box.downcast_ref::<Identifier>();

        // 如果为标识符
        if let Some(it) = ident {
            let func = find_function(it.to_string(), args.clone(), env);

            return match func {
                Ok(mut func) => {
                    if let Some(it) = (func.clone() as Box<dyn Any>).downcast_ref::<AntFunction>() {
                        // 先与形参环境融合，防止指定参数时参数不在环境中
                        func.get_env_ref().in_place_fusion(it.param_env.to_owned());

                        // 将所有参数指定（赋值）表达式进行求值
                        eval_expressions(&assignment_expressions, evaluator, func.get_env_ref());

                        // 清除所有没被设置值的参数
                        func.get_env_ref().remove_obj(uninit_obj.clone());

                        Some(call_function(func, args, evaluator, env))
                    } else if let Some(it) = (func.clone() as Box<dyn Any>).downcast_ref::<AntNativeFunction>() {
                        // 先与形参环境融合，防止指定参数时参数不在环境中
                        func.get_env_ref().in_place_fusion(it.param_env.to_owned());

                        // 将所有参数指定（赋值）表达式进行求值
                        eval_expressions(&assignment_expressions, evaluator, func.get_env_ref());

                        // 清除所有没被设置值的参数
                        func.get_env_ref().remove_obj(uninit_obj.clone());

                        call_native_function(func, args, env)
                    } else {
                        None
                    }
                }

                Err(it) => {
                    Some(it)
                }
            }
        }

        let func_expr: Option<&FunctionExpression> = converted_box.downcast_ref::<FunctionExpression>();
        if let Some(mut it) = func_expr.cloned() {
            // 如果为函数表达式

            // 获取函数
            let obj = it.eval(evaluator, env)?;

            let mut func = (*obj.as_any().downcast_ref::<AntFunction>()?).to_owned();
            let func: &mut AntFunction = &mut func;

            // 先与形参环境融合，防止指定参数时参数不在环境中
            func.to_owned().get_env_ref().in_place_fusion(func.param_env.to_owned());

            // 将所有参数指定（赋值）表达式进行求值
            eval_expressions(&assignment_expressions, evaluator, func.get_env_ref());

            // 清除所有没被设置值的参数
            func.get_env_ref().remove_obj(uninit_obj.clone());

            // 求值
            return Some(call_function(Box::new(func.to_owned()), args, evaluator, env));
        };

        Some(create_error(format!("unsupported function expression: {}", self.func.to_string())))
    }
}

impl Expression for CallExpression {}

pub fn create_call_expression(token: Token, func: Box<dyn Expression>, args: Vec<Box<dyn Expression>>) -> CallExpression {
    CallExpression {
        token, func, args
    }
}