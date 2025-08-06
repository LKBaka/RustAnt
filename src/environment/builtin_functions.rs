use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use bigdecimal::{BigDecimal, FromPrimitive};

use crate::environment::environment::Environment;
use crate::environment::utils::create_env;
use crate::extract_arg;
use crate::object::ant_class::AntClass;
use crate::object::ant_double::AntDouble;
use crate::object::ant_int::AntInt;
use crate::object::ant_native_function::create_ant_native_function;
use crate::object::ant_string::AntString;
use crate::object::object::Object;
use crate::object::utils::create_error_with_name;
use crate::utils::run_command;

pub fn builtin_print(arg_env: &mut Environment) -> Option<Object> {
    let value = arg_env.get("value").expect(
        &format!("cannot find 'value'. arg_env: {}", arg_env.to_string())
    );

    let end = arg_env.get("end").expect(
        &format!("cannot find 'end'. arg_env: {}", arg_env.to_string())
    );

    print!("{}{}", value.inspect(), end.inspect());

    None
}

pub fn builtin_input(arg_env: &mut Environment) -> Option<Object> {
    let value = arg_env.get("prompt");
    match value {
        None => {
            let mut input = "".to_string();

            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            Some(AntString::new_with_native_value(Box::new(input)))
        }
        Some(it) => {
            print!("{}", it.inspect());

            let mut input = "".to_string();

            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            Some(AntString::new_with_native_value(Box::new(input)))
        }
    }
}

pub fn builtin_clear(_arg_env: &mut Environment) -> Option<Object> {
    if cfg!(target_os = "windows") {
        let result = run_command("cls");
    } else if cfg!(target_os = "linux") {
        let result = run_command("clear");
    }

    None
}

pub fn builtin_shell(arg_env: &mut Environment) -> Option<Object> {
    let command = extract_arg!(arg_env, "command" => AntString)?;

    let result = run_command(command.value.as_str());

    None
}

pub fn builtin_now(_arg_env: &mut Environment) -> Option<Object> {
    Some(
        AntDouble::new_with_native_value(Box::new(BigDecimal::from_f64(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| {
                    duration
                        .as_secs() as f64
                        + duration
                            .subsec_nanos() as f64
                            / 1_000_000_000.0
                })
                .unwrap_or(-1.0)
        ).unwrap()))
    )
}

pub fn builtin_range(arg_env: &mut Environment) -> Option<Object> {
    let end = extract_arg!(arg_env, "end" => AntInt)?;

    let mut range_class = AntClass {
        id: uuid::Uuid::new_v4(),
        name: "Range".to_string(),
        base: {
            let iterable_class = arg_env.get("Iterable").expect(&format!(
                "cannot find Iterable class in environment: {}",
                arg_env.to_string()
            ));

            Some(iterable_class)
        },
        env: create_env(vec![
            ("start".to_string(), Box::new(AntInt::from(0))),
            ("end".to_string(), Box::new(end)),
            ("step".to_string(), Box::new(AntInt::from(1))),
            ("current".to_string(), Box::new(AntInt::from(1))),
        ])
    };

    let iter_function = {
        let native_function = |arg_env: &mut Environment| -> Option<Object> {
            let me = extract_arg!(arg_env, "me" => AntClass)?;

            return Some(Box::new(me))
        };

        let param_env = create_env(vec![
            ("me".to_string(), Box::new(range_class.clone())),
        ]);

        create_ant_native_function(param_env, None, native_function)
    };

    let next_function = {
        let native_function = |arg_env: &mut Environment| -> Option<Object> {
            let me = extract_arg!(arg_env, "me" => AntClass)?;

            let start_obj = extract_arg!(me.env, "start" => AntInt)?;
            let end_obj = extract_arg!(me.env, "end" => AntInt)?;
            let step_obj = extract_arg!(me.env, "step" => AntInt)?;
            let current_obj = extract_arg!(me.env, "current" => AntInt)?;

            let start = start_obj.value.clone();
            let end = end_obj.value.clone(); 
            let step = step_obj.value.clone();

            let mut current = current_obj.value.clone();

            if start >= end {
                return Some(create_error_with_name("StopError", "".into())); // 迭代结束 抛出错误
            }

            current = if current == BigDecimal::from(0) {
                start.clone()
            } else {
                current + step.clone()
            };

            // 修改原来的对象
            
            
            return Some(Box::new(AntInt::from(current)));
        };

        let param_env = create_env(vec![
            ("me".to_string(), Box::new(range_class.clone())),
        ]);

        create_ant_native_function(param_env, None, native_function)
    };

    None
}