use crate::ast::ast::Node;
use crate::constants::{null_obj, uninit_obj};
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::object::ant_error::AntError;
use crate::object::ant_function::AntFunction;
use crate::object::ant_native_function::AntNativeFunction;
use crate::object::ant_return_value::AntReturnValue;
use crate::object::object::{IAntObject, RETURN_VALUE, UNINIT};

fn check_function_args_count(param_env: Environment, env: &mut Environment, args: Vec<Box<dyn IAntObject>>) -> Result<i32, Box<dyn IAntObject>>{
    if args.len() == param_env.map.keys().len() {
        // 若实参数量与形参数量相同，则按位置陆续传输参数
        for i in 0..args.len() {
            let name = param_env.map.keys()[i].clone();
            let value = args[i].clone();

            env.create(&name, Data::new(value, DataInfo::new(false)));
        }

        return Ok(0)
    }

    if args.len() > param_env.map.keys().len() {
        // 提供实参过多，报错
        return Err(
            AntError::new_with_native_value(Box::new(
                format!("This function requires {} parameter, but is given {} parameters", param_env.map.keys().len(), args.len())
            ))
        )
    }

    // 获取所有必须提供的形参
    let required_given_params = param_env.map.filter(|_name, data| {data.data.get_type() == UNINIT.to_string() });

    if args.len() < required_given_params.keys().len() {
        // 提供实参过少，报错
        return Err(
            AntError::new_with_native_value(Box::new(
                format!("This function requires {} parameter, but is given {} parameters", param_env.map.keys().len(), args.len())
            ))
        )
    }

    // 填充参数
    for (i, pair) in param_env.map.pairs.iter().enumerate() {
        let name = pair.key.clone();
        let default_value = pair.value.data.clone();

        if i < args.len() {
            // 如果有对应的实参，使用实参值
            env.create(&name, Data::new(args[i].clone(), DataInfo::new(false)));
        } else {
            // 如果没有对应的实参，使用默认值
            env.create(&name, Data::new(default_value, DataInfo::new(false)));
        }
    }

    Ok(0)
}

pub fn call_function_with_name(name: String, args: Vec<Box<dyn IAntObject>>, env: &mut Environment) -> Option<Box<dyn IAntObject>> {
    let try_get_functions = env.get_values(&*name);

    match try_get_functions {
        None => {
            Some(
                AntError::new_with_native_value(
                    Box::new(
                        format!("cannot find function \"{}\"", name)
                    )
                )
            )
        }
        Some(functions) => {
            // 逐个检验函数是否符合要求
            for function in functions {
                let converted_function = function.as_any().downcast_ref::<AntFunction>();
                if converted_function.is_some() {
                    let mut func: AntFunction = (*converted_function.unwrap()).clone();

                    let check_result = check_function_args_count(
                        func.param_env, &mut func.env, args.clone()
                    );

                    return match check_result {
                        Ok(_) => {
                            Some(call_function(function, args.clone(), env))
                        }

                        Err(obj) => {
                            Some(obj)
                        }
                    }
                }

                let converted_native_function = function.as_any().downcast_ref::<AntNativeFunction>();
                if converted_native_function.is_some() {
                    let mut func = (*converted_native_function.unwrap()).clone();

                    let check_result = check_function_args_count(
                        func.param_env, &mut func.env, args.clone()
                    );

                    return match check_result {
                        Ok(_) => {
                            call_native_function(function, args.clone(), env)
                        }

                        Err(obj) => {
                            Some(obj)
                        }
                    }
                }

                return Some(AntError::new_with_native_value(
                    Box::new(
                        format!("object {} is not callable", converted_function.unwrap().inspect())
                    )
                ))
            }

            Some(null_obj.clone())
        }
    }
}

pub fn call_native_function_with_arg_env(function: Box<dyn IAntObject>, arg_env: Environment, env: &Environment) -> Option<Box<dyn IAntObject>> {
    let converted_function = function.as_any().downcast_ref::<AntNativeFunction>();
    if converted_function.is_none() {
        return Some(AntError::new_with_native_value(
            Box::new(
                format!("object {} is not callable", converted_function.unwrap().inspect())
            )
        ))
    }

    let mut func = (*converted_function.unwrap()).clone();
    func.env.outer = Some(Box::new(env.clone()));

    // 检查参数数量
    // 待定

    func.env = func.param_env.clone().fusion(arg_env.clone());
    func.env.remove_obj(uninit_obj.clone());

    func.function.clone()(&mut func.env.clone())
}


pub fn call_native_function(function: Box<dyn IAntObject>, args: Vec<Box<dyn IAntObject>>, env: &Environment) -> Option<Box<dyn IAntObject>> {
    let converted_function = function.as_any().downcast_ref::<AntNativeFunction>();
    if converted_function.is_none() {
        return Some(AntError::new_with_native_value(
            Box::new(
                format!("object {} is not callable", converted_function.unwrap().inspect())
            )
        ))
    }

    let mut func = (*converted_function.unwrap()).clone();
    func.env.outer = Some(Box::new(env.clone()));

    // 检查参数数量
    check_function_args_count(func.param_env.clone(), &mut func.env, args);

    func.function.clone()(&mut func.env.clone())
}

pub fn call_function(function: Box<dyn IAntObject>, args: Vec<Box<dyn IAntObject>>, env: &Environment) -> Box<dyn IAntObject>{
    let converted_function = function.as_any().downcast_ref::<AntFunction>();
    if converted_function.is_none() {
        return AntError::new_with_native_value(
            Box::new(
                format!("object {} is not callable", converted_function.unwrap().inspect())
            )
        )
    }

    let mut func = (*converted_function.unwrap()).clone();
    func.env.outer = Some(Box::new(env.clone()));

    // 检查参数数量
    check_function_args_count(func.param_env.clone(), &mut func.env, args);

    eval_function(Box::new(func))
}

pub fn eval_function(function: Box<dyn IAntObject>) -> Box<dyn IAntObject> {
    let function = function.as_any().downcast_ref::<AntFunction>();
    if function.is_none() {
        return AntError::new_with_native_value(
            Box::new(
                format!("object {} is not callable", function.unwrap().inspect())
            )
        );
    }

    let return_value = function.unwrap().clone().block.eval(
        &mut (function.unwrap().clone().env.clone())
    );

    unwrap_return_value(
        if return_value.is_some() { return_value.unwrap() } else {null_obj.clone()}
    )
}

pub fn unwrap_return_value(return_value: Box<dyn IAntObject>) -> Box<dyn IAntObject> {
    if return_value.get_type() == RETURN_VALUE {
        return return_value.as_any().downcast_ref::<AntReturnValue>().unwrap().clone().value
    }

    return_value
}