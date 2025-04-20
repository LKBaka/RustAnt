use std::ops::Deref;
use crate::constants::uninit_obj;
use crate::environment::builtin_functions::{builtin_input, builtin_print};
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::object::ant_native_function::create_ant_native_function;
use crate::object::object::IAntObject;

pub fn create_env(map: Vec<(String, Box<dyn IAntObject>)>) -> Environment {
    let mut env = Environment::new();

    for (name, value) in map {
        env.create(name.deref(), Data::new(value, DataInfo::new(false)));
    }

    env
}

fn add_builtin_functions(env: &mut Environment) {
    let print_function = {
        let mut param_env = Environment::new();
        param_env.create("value", Data::new(uninit_obj.clone(), DataInfo::new(false)));

        create_ant_native_function(param_env, builtin_print)
    };

    let input_function = {
        let mut param_env = Environment::new();
        param_env.create("prompt", Data::new(uninit_obj.clone(), DataInfo::new(false)));

        create_ant_native_function(param_env, builtin_input)
    };

    env.create("print", Data::new(print_function, DataInfo::new(false)));
    env.create("input", Data::new(input_function, DataInfo::new(false)));
}

pub fn create_top_env() -> Environment {
    let mut env = Environment::new();

    add_builtin_functions(&mut env);

    env
}