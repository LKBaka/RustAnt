use std::ops::Deref;
use std::vec;
use crate::constants::uninit_obj;
use crate::environment::builtin_functions::{builtin_input, builtin_print};
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::object::ant_native_function::create_ant_native_function;
use crate::object::ant_string::AntString;
use crate::object::object::Object;

use super::builtin_functions::{builtin_clear, builtin_now, builtin_shell};

pub fn create_env(map: Vec<(String, Object)>) -> Environment {
    let mut env = Environment::new();

    for (name, value) in map {
        env.create(name.deref(), Data::new(value, DataInfo::new(false)));
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

        create_ant_native_function(param_env, builtin_print)
    };

    let input_function = {
        let param_env = create_env(
            vec![
                ("prompt".to_string(), AntString::new_with_native_value(Box::new("".to_string()))),
            ]
        );

        create_ant_native_function(param_env, builtin_input)
    };

    let shell_function = {
        let param_env = create_env(
            vec![
                ("command".to_string(), uninit_obj.clone()),
            ]
        );

        create_ant_native_function(param_env, builtin_shell)
    };

    let clear_function = {
        let param_env = create_env(vec![]);

        create_ant_native_function(param_env, builtin_clear)
    };

    let now_function = {
        let param_env = create_env(vec![]);

        create_ant_native_function(param_env, builtin_now)
    };

    env.create("print", Data::new(print_function, DataInfo::new(false)));
    env.create("input", Data::new(input_function, DataInfo::new(false)));
    env.create("shell", Data::new(shell_function, DataInfo::new(false)));
    env.create("clear", Data::new(clear_function, DataInfo::new(false)));
    env.create("now", Data::new(now_function, DataInfo::new(false)));
}

pub fn create_top_env() -> Environment {
    let mut env = Environment::new();

    add_builtin_functions(&mut env);

    env
}