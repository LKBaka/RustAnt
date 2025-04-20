use std::ops::Deref;

use crate::ast::ast::{Expression, Node};
use crate::constants::null_obj;
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::function_caller::function_caller::call_native_function_with_arg_env;
use crate::object::ant_function::AntFunction;
use crate::object::ant_native_function::AntNativeFunction;
use crate::object::object::{IAntObject, NATIVE_FUNCTION};
use crate::token::token::Token;

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
    func: Box<dyn Expression>,
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

    fn eval(&mut self, env: &mut Environment) -> Option<Box<(dyn IAntObject + 'static)>> {
        let mut arg_env = Environment::new_with_outer((*&env).clone());

        // 处理参数
        for i in 0..self.args.len() {
            if self.args.is_empty() {
                break;
            }

            let eval_result = self.func.eval(&mut env.clone());

            match eval_result {
                None => {}
                Some(eval_result) => {
                    let func = eval_result.as_any().downcast_ref::<AntFunction>();
                    if let Some(it) = func.cloned() {
                        let value = self.args[i].eval(env);
                        match value {
                            None => {},
                            Some(value) => {
                                arg_env.create(it.param_env.map.keys()[i].deref(), Data::new(value, DataInfo::new(false)));
                            }
                        }
                    }

                    let native_func = eval_result.as_any().downcast_ref::<AntNativeFunction>();
                    if let Some(it) = native_func.cloned() {
                        let value = self.args[i].eval(env);
                        match value {
                            None => {},
                            Some(value) => {
                                arg_env.create(it.param_env.map.keys()[i].deref(), Data::new(value, DataInfo::new(false)));
                            }
                        }
                    }
                }
            }
        }

        let function = self.func.eval(env);
        if function.clone()?.get_type() == NATIVE_FUNCTION {
            return Some(call_native_function_with_arg_env(function?, arg_env, env)?);
        }

        Some(null_obj.clone())
    }
}

impl Expression for CallExpression {}

pub fn create_call_expression(token: Token, func: Box<dyn Expression>, args: Vec<Box<dyn Expression>>) -> CallExpression {
    CallExpression {
        token, func, args
    }
}