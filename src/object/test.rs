#[cfg(test)]
mod tests {
    use colored::Colorize;

    use crate::{byte_code_vm::constants::{UNINIT_OBJ, UNINIT_OBJECT}, obj_enum::object::Object, object::{ant_string::AntString, object::IAntObject}};

    fn test_object_inspect(obj: Object, expected_inspect: String) {
        let inspected = obj.inspect();

        if inspected != expected_inspect {
            panic!(
                "Expected inspect result is {}, but now it is {}",
                expected_inspect, inspected
            );
        }

        println!("{}", inspected);
    }

    #[test]
    fn test_objects_inspect() {
        use crate::object::ant_int::AntInt;

        let cases: Vec<(Object, String)> = vec![
            (Object::AntInt(AntInt::from(91)), String::from("91")),
            (Object::AntInt(AntInt::from(78)), String::from("78")),
            (
                Object::AntUninit(UNINIT_OBJ.clone()),
                String::from("uninit"),
            ),
        ];

        for (key, value) in cases {
            test_object_inspect(key, value);
        }
    }

    #[derive(Debug)]
    enum CompareOp {
        Eq,
        NotEq,
    }

    fn test_object_compare(
        op: CompareOp,
        left: Object,
        right: Object,
    ) {
        let result = match op {
            CompareOp::Eq => left == right,
            CompareOp::NotEq => left != right,
        };

        if !result {
            panic!(
                "expected {} {:?} {}, but they are not {:?}",
                left.inspect(), op, right.inspect(), op 
            )
        }

        println!(
            "{}",
            format!(
                "{} {:?} {}", 
                left.inspect(), op, right.inspect() 
            ).green()
        );
    }

    #[test]
    fn test_objects_compare() {
        let cases = vec![
            (
                Object::AntString(AntString::new(String::from("1\r\n"))),
                UNINIT_OBJECT.clone(),
                CompareOp::NotEq
            ),
            (
                Object::AntString(AntString::new(String::from("1\r\n"))),
                Object::AntString(AntString::new(String::from("1\r\n"))),
                CompareOp::Eq
            )
        ];

        for (l, r, op) in cases {
            test_object_compare(op, l, r);
        }
    }
}
