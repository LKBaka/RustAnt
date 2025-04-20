#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]

use crate::object::object::IAntObject;

fn test_object_inspect(obj: Box<impl IAntObject + ?Sized>, expected_inspect: String) {
    let inspected = obj.inspect();

    if inspected != expected_inspect {
        panic!("Expected inspect result is {}, but now it is {}", expected_inspect, inspected);
    }

    println!("{}", inspected);
}

#[test]
fn test_objects_inspect() {
    use num_bigint::BigInt;

    use crate::constants::null_obj;
    use crate::object::ant_int::AntInt;

    let cases = vec![
        (AntInt::new_with_native_value(Box::new(BigInt::from(91))), String::from("91")),
        (AntInt::new_with_native_value(Box::new(BigInt::from(78))), String::from("78")),
        (null_obj.clone(), String::from("null")),
    ];

    for (key, value) in cases {
        test_object_inspect(key, value);
    }
}