use std::ops::Deref;
use std::vec;

use crate::constants::uninit_obj;
use crate::environment::builtin_functions::{builtin_input, builtin_print, builtin_range};
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::object::ant_class::AntClass;
use crate::object::ant_int::{create_ant_int, AntInt};
use crate::object::ant_native_function::create_ant_native_function;
use crate::object::ant_string::AntString;
use crate::object::object::Object;
use crate::object::utils::{create_error, create_error_with_name};

use super::builtin_functions::{builtin_clear, builtin_now, builtin_shell};

use crate::object::object::{ANY, STRING};
use crate::object::type_hint::{TypeHint, TypeHintMap};
use crate::{extract_arg, type_hint, type_hint_map};

pub fn create_env(map: Vec<(String, Object)>) -> Environment {
    let mut env = Environment::new();

    for (name, value) in map {
        let result = 
            env.create(name.deref(), Data::new(value, DataInfo::new(false)));

        if let Some(err) = result {
            eprintln!("Error creating env variable '{}': {}", name, err.inspect()); 
        }    
    }

    env
}

fn add_builtin_functions(env: &mut Environment) {
    let print_function = {
        let param_env = create_env(
            vec![
                ("value".to_string(), uninit_obj.clone()),
                ("end".to_string(), AntString::new_with_native_value(Box::new("\n".to_string()))),
            ]
        );

        let type_hint_map = type_hint_map!(
            "value" => type_hint!(ANY),
            "end" => type_hint!(STRING)
        );

        create_ant_native_function(param_env, Some(type_hint_map), builtin_print)
    };

    let input_function = {
        let param_env = create_env(
            vec![
                ("prompt".to_string(), AntString::new_with_native_value(Box::new("".to_string()))),
            ]
        );

        let type_hint_map = type_hint_map!(
            "prompt" => type_hint!(STRING)
        );

        create_ant_native_function(param_env, Some(type_hint_map), builtin_input)
    };

    let shell_function = {
        let param_env = create_env(
            vec![
                ("command".to_string(), uninit_obj.clone()),
            ]
        );

        let type_hint_map = type_hint_map!(
            "command" => type_hint!(STRING)
        );

        create_ant_native_function(param_env, Some(type_hint_map), builtin_shell)
    };

    let clear_function = {
        let param_env = create_env(vec![]);

        create_ant_native_function(param_env, None, builtin_clear)
    };

    let now_function = {
        let param_env = create_env(vec![]);

        create_ant_native_function(param_env, None, builtin_now)
    };

    let range_function = {
        let param_env = create_env(vec![
            ("end".to_string(), uninit_obj.clone()),
        ]);

        create_ant_native_function(param_env, None, builtin_range)
    };

    env.create("print", Data::new(print_function, DataInfo::new(false)));
    env.create("input", Data::new(input_function, DataInfo::new(false)));
    env.create("shell", Data::new(shell_function, DataInfo::new(false)));
    env.create("clear", Data::new(clear_function, DataInfo::new(false)));
    env.create("now", Data::new(now_function, DataInfo::new(false)));
    env.create("range", Data::new(range_function, DataInfo::new(false)));
}

fn add_builtin_classes(env: &mut Environment) {
    let iterable_class = {
        AntClass {
            id: uuid::Uuid::new_v4(),
            name: "Iterable".to_string(),
            base: None,
            env: create_env(
                vec![
                    ("iter".to_string(), uninit_obj.clone()),
                ]
            ),
        }
    };

    let iterator_class = {
        AntClass {
            id: uuid::Uuid::new_v4(),
            name: "Iterator".to_string(),
            base: Some(Box::new(iterable_class.clone())),
            env: create_env(
                vec![
                    ("next".to_string(), uninit_obj.clone()),
                    ("iter".to_string(), uninit_obj.clone()),
                ]
            ),
        }
    };

    let error_class = {
        let throw_function = {
            let param_env = create_env(
                vec![
                    ("message".to_string(), uninit_obj.clone()),
                ]
            );

            let type_hint_map = type_hint_map!(
                "message" => type_hint!(STRING)
            );


            let native_func = |arg_env: &mut Environment| {
                let message = extract_arg!(arg_env, "message" => AntString);

                if let Some(msg) = message {
                    return Some(create_error(msg.value));
                }

                None
            };

            create_ant_native_function(
                param_env, Some(type_hint_map), native_func
            )
        };

        AntClass {
            id: uuid::Uuid::new_v4(),
            name: "Error".to_string(),
            base: None,
            env: create_env(
                vec![
                    ("throw".to_string(), throw_function),
                ]
            ),
        }
    };

    let not_implemented_error_class = {
        let throw_function = {
            let param_env = create_env(
                vec![
                    ("self".to_string(), uninit_obj.clone()),
                    ("message".to_string(), uninit_obj.clone()),
                ]
            );

            let type_hint_map = type_hint_map!(
                "message" => type_hint!(STRING)
            );


            let native_func = |arg_env: &mut Environment| {
                let message = extract_arg!(arg_env, "message" => AntString);

                if let Some(msg) = message {
                    return Some(create_error_with_name("NotImplementedError", msg.value));
                }

                None
            };

            create_ant_native_function(
                param_env, Some(type_hint_map), native_func
            )
        };

        AntClass {
            id: uuid::Uuid::new_v4(),
            name: "NotImplementedError".to_string(),
            base: None,
            env: create_env(
                vec![
                    ("throw".to_string(), throw_function),
                ]
            ),
        }
    };

    let classes_map = vec![
        ("Iterator", iterator_class),
        ("Iterable", iterable_class),
        ("Error", error_class),
        ("NotImplementedError", not_implemented_error_class),
    ];

    for (name, class) in classes_map {
        let result = env.create(name, Data::new(Box::new(class), DataInfo::new(false)));
        if let Some(err) = result {
            eprintln!("Error creating class '{}': {}", name, err.inspect());
        }
    }

}

pub fn create_top_env() -> Environment {
    let mut env = Environment::new();

    add_builtin_functions(&mut env);
    add_builtin_classes(&mut env);

    env
}