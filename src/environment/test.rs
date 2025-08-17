#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]

use std::ops::Deref;

use crate::environment::data::Data;
use crate::environment::environment::Environment;

fn test_env_set_value(mut env: Environment, key: String, value: Data) {
    env.set(key.deref(), value.clone());

    let get_result = env.get(key.deref()).unwrap();

    if !(&get_result == &value.data) {
        panic!(
            "{}",
            format!(
                "Expected inspect result is {}, but now it is {}",
                value.data.inspect(),
                get_result.inspect()
            )
        );
    }

    println!(
        "{}",
        format!(
            "result: {} == expected: {}",
            get_result.deref().inspect(),
            value.data.inspect()
        )
    );
}

fn test_env_fusion(env1: Environment, env2: Environment, expected_env: Environment) {
    let new_env = env1.fusion(env2.clone());

    if !new_env.clone().eq(&expected_env.clone()) {
        panic!(
            "{}",
            format!(
                "Expected fusion result is {}, but now it is {}",
                expected_env.to_string(),
                new_env.to_string()
            )
        )
    }

    println!(
        "{}",
        format!(
            "result: {} == expected: {}",
            new_env.to_string(),
            expected_env.clone().to_string()
        )
    );
}

#[test]
fn test_env_set_values() {
    use crate::constants::null_obj;
    use crate::environment::data_info::DataInfo;
    use crate::object::ant_int::AntInt;
    use crate::object::object::Object;

    let mut env = Environment::new();

    let cases = vec![
        ("null", null_obj.clone()),
        ("zero", Box::new(AntInt::from(0))),
    ];

    for (key, value) in cases {
        let data = Data::new(value, DataInfo::new(false));
        env.create(key, data.clone());

        test_env_set_value(env.clone(), key.to_string(), data.clone());
    }
}

#[test]
fn test_multi_env_fusion() {
    use crate::constants::uninit_obj;
    use crate::environment::data_info::DataInfo;
    use crate::object::ant_int::AntInt;
    use crate::object::object::Object;

    // 定义测试数据表
    let env1_items = vec![("null", uninit_obj.clone()), ("zero", uninit_obj.clone())];

    let env2_items = vec![
        ("null", Box::new(AntInt::from(0)) as Object),
        ("zero", uninit_obj.clone()),
    ];

    // 初始化环境
    let mut env1 = Environment::new();
    let mut env2 = Environment::new();

    // 填充 env1
    for (key, value) in &env1_items {
        env1.create(key, Data::new(value.clone(), DataInfo::new(false)));
    }

    // 填充 env2
    for (key, value) in &env2_items {
        env2.create(key, Data::new(value.clone(), DataInfo::new(false)));
    }

    test_env_fusion(env1.clone(), env2.clone(), env1.fusion(env2.clone()))
}
