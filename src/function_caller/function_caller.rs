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
use crate::object::function_enum::Function;
use crate::object::object::{Object, ANY, UNINIT};
use crate::object::type_hint::TypeHintMap;
use crate::object::utils::{create_error, create_error_with_name, is_error, type_error};
use crate::rc_ref_cell;

pub fn fill_args(args: &Vec<&Object>, param_env: &Environment, env: &mut Environment) -> Result<(), Object> {
    let args_len = args.len();

    for (i, pair) in param_env.map.pairs.iter().enumerate() {
        let name = &pair.key;

        if i < args_len {
            if is_error(&args[i]) {return Err(args[i].clone())}

            // 如果有对应的实参，使用实参值
            env.create(name, Data::new(args[i].clone(), DataInfo::new(false)));
            continue
        }

        // 如果没有对应的实参，使用默认值
        let default_value = pair.value.data.clone();

        env.create(name, Data::new(default_value, DataInfo::new(false))); 
    }

    Ok(())
}

fn check_function_args_type(arg_env: &Environment, type_hint_map: &TypeHintMap) -> Result<(), Object> {
    for pair in &arg_env.map.pairs {
        let name = &pair.key;
        let data = &pair.value;

        let type_hint = type_hint_map.get_hint(name);
        if let Some(type_hint) = type_hint {
            if type_hint.has_type(&data.data.get_type()) || type_hint.has_type(&ANY.into()) {
                continue
            }

            return Err(type_error(&format!(
                "type mismatch for argument '{}': expected one of {:?}, found {}",
                name, type_hint.types, data.data.get_type()
            )));
        }
    }

    Ok(())
}

fn not_callable_error(obj_inspected: String) -> Object {
    create_error(
        format!("object {} is not callable", obj_inspected)
    )
}

fn check_function_args_count(param_env: &Environment, env: &mut Environment, args: &Vec<&Object>) -> Result<(), Object>{
    let args_len = args.len();
    
    if args_len == param_env.map.keys().len() {
        // 若实参数量与形参数量相同，则按位置陆续传输参数
        for i in 0..args_len {
            let name = &param_env.map.keys()[i];
            let value: Object = args[i].clone();

            if is_error(&value) {return Err(value)}

            let result = env.create(name, Data::new(value.clone(), DataInfo::new(false)));
            if result.is_some() {
                env.set_value(name, value);
            }
        }

        return Ok(())
    }

    // 获取所有必须提供的形参
    let required_given_params = param_env.map
        .filter(|_name, data| {data.data.get_type() == UNINIT });

    if args_len > param_env.map.keys().len() {
        // 提供实参不符合预期，报错
        return Err(
            create_error(
                format!("this function max requires {} parameter, but is given {} parameters", param_env.map.keys().len(), args_len)
            )
        )
    }

    if args_len < required_given_params.keys().len() {
        // 提供实参不符合预期，报错
        return Err(
            create_error(
                format!("this function requires {} parameter, but is given {} parameters", required_given_params.keys().len(), args_len)
            )
        )
    }

    // 填充参数
    let fill_result = fill_args(args, param_env, env);

    fill_result
}

pub fn find_function(name: String, args: &Vec<&Object>, env: &mut Environment) -> Result<Object, Object> {
    let functions = match env.get_values(&*name) {
        Some(funcs) => funcs,
        None => return Err(create_error(format!("cannot find function \"{}\"", name))),
    };

    for function in functions {
        let converted_function = function.as_any().downcast_ref::<AntFunction>();
        if let Some(func) = converted_function {
            let check_result = check_function_args_count(
                &func.param_env, &mut func.env.clone(), args
            );

            if let Ok(_) = check_result {return Ok(function)}
        }

        let converted_native_function = function.as_any().downcast_ref::<AntNativeFunction>();
        if let Some(func) = converted_native_function {
            let check_result = check_function_args_count(
                &func.param_env, &mut func.env.clone(), args
            );

            if let Ok(_) = check_result {return Ok(function)}
        }
    }

    Err(
        create_error("no matching function overload found.".to_string())
    )
}


pub fn call_function_with_name(name: String, args: &Vec<&Object>, evaluator: &mut Evaluator, env: &mut Environment) -> Result<Option<Object>, Object> {
    let func = find_function(name, args, env);

    match func {
        Ok(func) => {
            if (func.as_ref() as &dyn Any).is::<AntFunction>() {
                Ok(Some(call_function(Function::Func(func), &args, evaluator, env)))
            } else if (func.as_ref() as &dyn Any).is::<AntNativeFunction>() {
                Ok(call_native_function(Function::NativeFunc(func), &args, env))
            } else {
                Err(not_callable_error(func.inspect()))
            }
        }

        Err(err) => {
            Err(err)
        }
    }
}

pub fn call_native_function(function: Function, args: &Vec<&Object>, env: &Environment) -> Option<Object> {
    let converted_function = match function {
        Function::Func(func) => {
            return Some(create_error_with_name("TypeError", format!(
                "function {} is not a native function",
                func.inspect()
            )));
        },
        Function::NativeFunc(func) =>
            func
                .as_any()
                .downcast_ref::<AntNativeFunction>()
                .expect("not a native function")
                .clone()
    };

    let mut func = converted_function;
    func.env.outer = Some(rc_ref_cell!(env.clone()));

    // 检查参数数量
    let result = check_function_args_count(&func.param_env, &mut func.env, args);
    if let Err(err) = result {
        return Some(err)
    }

    if let Some(type_hint_map) = &func.type_hint_map {
        let check_type_result = check_function_args_type(
            &func.env, type_hint_map
        );

        if let Err(it) = check_type_result {
            return Some(it);
        }
    }

    (func.function)(&mut func.env)
}

pub fn call_function(function: Function, args: &Vec<&Object>, evaluator: &mut Evaluator, env: &mut Environment) -> Object {
    let converted_function = match function {
        Function::Func(func) => 
            func
                .as_any()
                .downcast_ref::<AntFunction>()
                .expect("not a function")
                .clone()
        ,
        Function::NativeFunc(func) => {
            return call_native_function(Function::NativeFunc(func), args, env).unwrap_or(null_obj.clone())
        }
    };

    let mut func = converted_function;
    func.env.outer = Some(rc_ref_cell!(env.clone()));

    // 检查参数数量
    let result = check_function_args_count(&func.param_env, &mut func.env, args);
    if let Err(err) = result {
        return err
    }

    eval_function(evaluator, Box::new(func))
}

pub fn eval_function(evaluator: &mut Evaluator, function: Object) -> Object {
    let func = function.as_any().downcast_ref::<AntFunction>();

    if let Some(func) = func.cloned() {
        let mut func_env = func.env;
        let mut block = func.block;

        let return_value = block.eval(
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
    if let Some(return_value) = return_value.as_any().downcast_ref::<AntReturnValue>() {
        return return_value.value.clone();
    }

    return_value
}