use std::ops::Deref;
use crate::environment::data::Data;
use crate::environment::data_info::DataInfo;
use crate::environment::environment::Environment;
use crate::object::object::Object;

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
