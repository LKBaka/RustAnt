use std::any::Any;

use crate::ast::ast::Node;
use crate::constants::null_obj;
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::evaluator::evaluator::Evaluator;
use crate::object::ant_function::AntFunction;
use crate::object::ant_native_function::AntNativeFunction;
use crate::object::ant_return_value::AntReturnValue;
use crate::object::object::{Object, RETURN_VALUE, UNINIT};
use crate::object::utils::{create_error, is_error};

pub fn fill_args(args: Vec<Object>, param_env: Environment, env: &mut Environment) -> Result<(), Object> {
    for (i, pair) in param_env.map.pairs.iter().enumerate() {
        let name = pair.key.clone();
        let default_value = pair.value.data.clone();

        if i < args.len() {
            if is_error(&args[i]) {return Err(args[i].clone())}

            // 如果有对应的实参，使用实参值
            env.create(&name, Data::new(args[i].clone(), DataInfo::new(false)));
            continue
        }

        // 如果没有对应的实参，使用默认值
        env.create(&name, Data::new(default_value, DataInfo::new(false))); 
    }

    Ok(())
}

fn not_callable_error(obj_inspected: String) -> Object {
    create_error(
        format!("object {} is not callable", obj_inspected)
    )
}

fn check_function_args_count(param_env: Environment, env: &mut Environment, args: Vec<Object>) -> Result<(), Object>{
    if args.len() == param_env.map.keys().len() {
        // 若实参数量与形参数量相同，则按位置陆续传输参数
        for i in 0..args.len() {
            let name = param_env.map.keys()[i].clone();
            let value: Object = args[i].clone();

            if is_error(&value) {return Err(value)}

            let result = env.create(&name, Data::new(value.to_owned(), DataInfo::new(false)));
            if result.is_some() {
                env.set_value(&name, value);
            }
        }

        return Ok(())
    }

    // 获取所有必须提供的形参
    let required_given_params = param_env.map.filter(|_name, data| {data.data.get_type() == UNINIT.to_string() });

    if args.len() > param_env.map.keys().len() {
        // 提供实参不符合预期，报错
        return Err(
            create_error(
                format!("this function max requires {} parameter, but is given {} parameters", param_env.map.keys().len(), args.len())
            )
        )
    }

    if args.len() < required_given_params.keys().len() {
        // 提供实参不符合预期，报错
        return Err(
            create_error(
                format!("this function requires {} parameter, but is given {} parameters", required_given_params.keys().len(), args.len())
            )
        )
    }

    // 填充参数
    let fill_result = fill_args(args, param_env, env);

    fill_result
}

pub fn find_function(name: String, args: Vec<Object>, env: &mut Environment) -> Result<Object, Object> {
    let functions = match env.get_values(&*name) {
        Some(funcs) => funcs,
        None => return Err(create_error(format!("cannot find function \"{}\"", name))),
    };

    for function in functions {
        let converted_function = function.as_any().downcast_ref::<AntFunction>();
        if let Some(func) = converted_function {
            let check_result = check_function_args_count(
                func.param_env.to_owned(), &mut func.env.to_owned(), args.clone()
            );

            if let Ok(_) = check_result {return Ok(function)}
        }

        let converted_native_function = function.as_any().downcast_ref::<AntNativeFunction>();
        if let Some(func) = converted_native_function {
            let check_result = check_function_args_count(
                func.param_env.to_owned(), &mut func.env.to_owned(), args.clone()
            );

            if let Ok(_) = check_result {return Ok(function)}
            if let Err(it) = check_result {return Err(it)}
        }
    }

    Err(
        create_error("no matching function overload found.".to_string())
    )
}

pub fn call_function_with_name(name: String, args: Vec<Object>, evaluator: &mut Evaluator, env: &mut Environment) -> Result<Option<Object>, Object> {
    let func = find_function(name, args.clone(), env);

    if let Ok(func) = func {
        return if let Some(_) = (func.to_owned() as Box<dyn Any>).downcast_ref::<AntFunction>() {
            Ok(Some(call_function(func, args, evaluator, env)))
        } else if let Some(_) = (func.to_owned() as Box<dyn Any>).downcast_ref::<AntNativeFunction>() {
            Ok(call_native_function(func, args, env))
        } else {
            Err(not_callable_error(func.inspect()))
        }
    }

    if let Err(err) = func {
        return Err(err)
    }

    Ok(Some(null_obj.clone())) // 不可能到达的代码
}

pub fn call_native_function(function: Object, args: Vec<Object>, env: &Environment) -> Option<Object> {
    let converted_function = function.as_any().downcast_ref::<AntNativeFunction>();
    if converted_function.is_none() {
        return Some(not_callable_error(function.inspect()))
    }

    let mut func = (converted_function.unwrap()).to_owned();
    func.env.outer = Some(Box::new(env.to_owned()));

    // 检查参数数量
    let result = check_function_args_count(func.param_env, &mut func.env, args);
    if let Err(err) = result {
        return Some(err)
    }

    func.function.clone()(&mut func.env.clone())
}

pub fn call_function(function: Object, args: Vec<Object>, evaluator: &mut Evaluator, env: &mut Environment) -> Object{
    let converted_function = function.as_any().downcast_ref::<AntFunction>();
    if converted_function.is_none() {
        return not_callable_error(function.inspect())
    }

    let mut func = (*converted_function.unwrap()).to_owned();
    func.env.outer = Some(Box::new(env.to_owned()));

    // 检查参数数量
    let result = check_function_args_count(func.param_env.clone(), &mut func.env, args);
    if let Err(err) = result {
        return err
    }

    eval_function(evaluator, Box::new(func))
}

pub fn eval_function(evaluator: &mut Evaluator, function: Object) -> Object {
    let func = function.as_any().downcast_ref::<AntFunction>();
    if let Some(mut func) = func.cloned() {
        let mut func_env = func.clone().env.clone();

        let return_value = (&mut func).block.eval(
            evaluator, 
            &mut func_env
        );

        return unwrap_return_value(
            if let Some(it) = return_value {it} else {null_obj.clone()}
        )
    }

    not_callable_error(function.inspect())
}

pub fn unwrap_return_value(return_value: Object) -> Object {
    if return_value.get_type() == RETURN_VALUE {
        return return_value.as_any().downcast_ref::<AntReturnValue>().unwrap().to_owned().value
    }

    return_value
}