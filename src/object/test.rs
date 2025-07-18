#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

use bigdecimal::BigDecimal;

use crate::{environment::utils::create_env, object::{ant_env::{create_ant_env, AntEnv}, object::IAntObject}};
use super::super::environment::environment::Environment;

fn test_object_inspect(obj: Box<impl IAntObject + ?Sized>, expected_inspect: String) {
    let inspected = obj.inspect();

    if inspected != expected_inspect {
        panic!("Expected inspect result is {}, but now it is {}", expected_inspect, inspected);
    }

    println!("{}", inspected);
}

#[test]
fn test_objects_inspect() {
    use crate::constants::null_obj;
    use crate::object::ant_int::AntInt;

    let cases = vec![
        (AntInt::new_with_native_value(Box::new(BigDecimal::from(91))), String::from("91")),
        (AntInt::new_with_native_value(Box::new(BigDecimal::from(78))), String::from("78")),
        (
            create_ant_env(create_env(vec![("a".into(), Box::new(AntInt::from(91)))])), 
            String::from("[a: Data(data: 91, data_info: DataInfo(readonly: false))]")
        ),
        (null_obj.clone(), String::from("null")),
    ];

    for (key, value) in cases {
        test_object_inspect(key, value);
    }
}