#[cfg(test)]
mod tests {
    use std::{
        any::{Any, TypeId},
        fmt::Debug,
        vec,
    };

    use bigdecimal::BigDecimal;
    use colored::Colorize;

    use crate::{
        big_dec, big_dec_from_str, byte_code_vm::{compiler::utils::compile_it, vm::vm::Vm}, convert_type_use_box, obj_enum::object::Object, object::{
            ant_array::AntArray,
            object::{IAntObject, DOUBLE, INT},
        }, try_unwrap
    };

    struct VmTestCase<T: Debug + Clone + 'static> {
        input: String,
        expected: T,
    }

    impl<T: Debug + Clone> VmTestCase<T> {
        pub fn new(input: String, expected: T) -> Self {
            Self { input, expected }
        }
    }

    #[test]
    fn test_integer_arithmetic() {
        let tests = vec![
            VmTestCase::<BigDecimal>::new("1".into(), big_dec!(1)),
            VmTestCase::<BigDecimal>::new("2".into(), big_dec!(2)),
            VmTestCase::<BigDecimal>::new("1 + 2".into(), big_dec!(3)),
            VmTestCase::<BigDecimal>::new("1 - 2".into(), big_dec!(-1)),
            VmTestCase::<BigDecimal>::new("1 * 2".into(), big_dec!(2)),
            VmTestCase::<BigDecimal>::new("4 / 2".into(), big_dec!(2)),
            VmTestCase::<BigDecimal>::new("50 / 2 * 2 + 10 - 5".into(), big_dec!(55)),
            VmTestCase::<BigDecimal>::new("5 + 5 + 5 + 5 - 10".into(), big_dec!(10)),
            VmTestCase::<BigDecimal>::new("50 / 2 * 2 + 10 - 5".into(), big_dec!(55)),
            VmTestCase::<BigDecimal>::new("2 * 2 * 2 * 2 * 2".into(), big_dec!(32)),
            VmTestCase::<BigDecimal>::new("5 * 2 + 10".into(), big_dec!(20)),
            VmTestCase::<BigDecimal>::new("5 + 2 * 10".into(), big_dec!(25)),
            VmTestCase::<BigDecimal>::new("5 * (2 + 10)".into(), big_dec!(60)),
            VmTestCase::<BigDecimal>::new("-5".into(), big_dec!(-5)),
            VmTestCase::<BigDecimal>::new("-10".into(), big_dec!(-10)),
            VmTestCase::<BigDecimal>::new("-50 + 100 + -50".into(), big_dec!(0)),
            VmTestCase::<BigDecimal>::new("(5 + 10 * 2 + 15 / 3) * 2 + -10".into(), big_dec!(50)),
            VmTestCase::<BigDecimal>::new("5 * (2 + 10)".into(), big_dec!(60)),
        ];

        run_vm_tests::<BigDecimal>(tests)
    }

    #[test]
    fn test_boolean_expressions() {
        let tests = vec![
            VmTestCase::<bool>::new("true".into(), true),
            VmTestCase::<bool>::new("false".into(), false),
            VmTestCase::<bool>::new("1 < 2".into(), true),
            VmTestCase::<bool>::new("1 > 2".into(), false),
            VmTestCase::<bool>::new("1 < 1".into(), false),
            VmTestCase::<bool>::new("1 > 1".into(), false),
            VmTestCase::<bool>::new("1 == 1".into(), true),
            VmTestCase::<bool>::new("1 != 1".into(), false),
            VmTestCase::<bool>::new("1 == 2".into(), false),
            VmTestCase::<bool>::new("1 != 2".into(), true),
            VmTestCase::<bool>::new("true == true".into(), true),
            VmTestCase::<bool>::new("false == false".into(), true),
            VmTestCase::<bool>::new("true == false".into(), false),
            VmTestCase::<bool>::new("true != false".into(), true),
            VmTestCase::<bool>::new("false != true".into(), true),
            VmTestCase::<bool>::new("(1 < 2) == true".into(), true),
            VmTestCase::<bool>::new("(1 < 2) == false".into(), false),
            VmTestCase::<bool>::new("(1 > 2) == true".into(), false),
            VmTestCase::<bool>::new("(1 > 2) == false".into(), true),
            VmTestCase::<bool>::new("!true".into(), false),
            VmTestCase::<bool>::new("!false".into(), true),
            VmTestCase::<bool>::new("!5".into(), false),
            VmTestCase::<bool>::new("!!true".into(), true),
            VmTestCase::<bool>::new("!!false".into(), false),
            VmTestCase::<bool>::new("!!5".into(), true),
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_none_literal() {
        let tests = vec![
            VmTestCase::<()>::new("none".into(), ()),
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_global_let_statements() {
        let tests = vec![
            VmTestCase::new("let one = 1; one".into(), big_dec!(1)),
            VmTestCase::new("let one = 1; let two = 2; one + two".into(), big_dec!(3)),
            VmTestCase::new(
                "let one = 1; let two = one + one; one + two".into(),
                big_dec!(3),
            ),
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_string_expressions() {
        let tests = vec![
            VmTestCase::new("\"lava\"".into(), String::from("lava")),
            VmTestCase::new("\"la\" + \"va\"".into(), String::from("lava")),
            VmTestCase::new(
                "\"la\" + \"va\" + \"hot!\"".into(),
                String::from("lavahot!"),
            ),
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_array_literal() {
        let tests = vec![
            VmTestCase::new("[]".into(), vec![]),
            VmTestCase::new(
                "[] + []".into(),
                vec![],
            ),
            VmTestCase::new(
                "[1, 2, 3]".into(),
                vec![big_dec!(1), big_dec!(2), big_dec!(3)],
            ),
            VmTestCase::new(
                "[1 + 2, 3 * 4, 5 + 6]".into(),
                vec![big_dec!(3), big_dec!(12), big_dec!(11)],
            ),
            VmTestCase::new(
                "[1, 2] + [3]".into(),
                vec![big_dec!(1), big_dec!(2), big_dec!(3)],
            ),
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_index_expression() {
        let tests = vec![
            VmTestCase::<BigDecimal>::new("[1, 2, 3][1]".into(), big_dec!(2)),
            VmTestCase::<BigDecimal>::new("[1, 2, 3][0 + 2]".into(), big_dec!(3)),
            VmTestCase::<BigDecimal>::new("[-1, -2, -3][-1]".into(), big_dec!(-3)),
            VmTestCase::<BigDecimal>::new("[-10, 0, 10][-3]".into(), big_dec!(-10)),
            VmTestCase::<BigDecimal>::new("[-0.5, -1.5, -2.5][-2]".into(), big_dec_from_str!(-1.5)),
            VmTestCase::<BigDecimal>::new("[100, -200, 300][-2]".into(), big_dec!(-200)),
            VmTestCase::<BigDecimal>::new("[-999, -888, -777][-3]".into(), big_dec!(-999)),
            VmTestCase::<BigDecimal>::new("[0.1, -0.2, 0.3][-1]".into(), big_dec_from_str!(0.3)),
            VmTestCase::<BigDecimal>::new("[-42][-1]".into(), big_dec!(-42)),
            VmTestCase::<BigDecimal>::new("[-1, 2, -3, 4, -5][-4]".into(), big_dec!(2)),
            VmTestCase::<BigDecimal>::new("[1.23, -4.56, 7.89][-2]".into(), big_dec_from_str!(-4.56)),
            VmTestCase::<BigDecimal>::new("[-1000, 2000, -3000][-1]".into(), big_dec!(-3000)),
            VmTestCase::<BigDecimal>::new("[[1, 1, 1]][0][0]".into(), big_dec!(1)),
            VmTestCase::<BigDecimal>::new("[[1, 4, 7]][-1][-1]".into(), big_dec!(7)),
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_function_calling() {
        let tests = vec![
            VmTestCase::new(
                r#"
                func five_plus_ten() { 5 + 10; }; 
                five_plus_ten(); 
                "#
                .into(),
                big_dec!(15),
            ),
            VmTestCase::new(
                r#"
                func five_plus_ten() { return 5 + 10; "vm has error!" }; 
                five_plus_ten(); 
                "#
                .into(),
                big_dec!(15),
            ),
            VmTestCase::new(
                r#"
                func one() { 1; }; 
                func two() { one() + one(); }; 
                func three() { two() + one() };
                three() + two() + one(); 
                "#
                .into(),
                big_dec!(6),
            ),
            // 测试1: 简单局部变量
            VmTestCase::<BigDecimal>::new(
                r#"
                func one() { let one = 1; one }; 
                one(); 
                "#
                .into(),
                big_dec!(1),
            ),
            // 测试2: 多个局部变量
            VmTestCase::<BigDecimal>::new(
                r#"
                func oneAndTwo() { let one = 1; let two = 2; one + two; }; 
                oneAndTwo(); 
                "#
                .into(),
                big_dec!(3),
            ),
            // 测试3: 多个函数调用
            VmTestCase::<BigDecimal>::new(
                r#"
                func oneAndTwo() { let one = 1; let two = 2; one + two; }; 
                func threeAndFour() { let three = 3; let four = 4; three + four; }; 
                oneAndTwo() + threeAndFour(); 
                "#
                .into(),
                big_dec!(10),
            ),
            // 测试4: 同名局部变量
            VmTestCase::<BigDecimal>::new(
                r#"
                func firstFoobar() { let foobar = 50; foobar; }; 
                func secondFoobar() { let foobar = 100; foobar; }; 
                firstFoobar() + secondFoobar(); 
                "#
                .into(),
                big_dec!(150),
            ),
            // 测试5: 闭包捕获外部变量
            VmTestCase::<BigDecimal>::new(
                r#"
                let globalSeed = 50; 
                func minusOne() { 
                    let num = 1; 
                    globalSeed - num; 
                }; 
                
                func minusTwo() { 
                    let num = 2; 
                    globalSeed - num; 
                }; 
                
                minusOne() + minusTwo(); 
                "#
                .into(),
                big_dec!(97),
            ),
            VmTestCase::new(
                r#"
                func f(val) {
                    val
                }

                f(1)
                "#
                .into(),
                big_dec!(1),
            ),
            VmTestCase::new(
                r#"
                func counter(val) {
                    if val > 100 {
                        val
                    } else {
                        let big_num = 9999999999999999999999 * 999999999999999999999999999;
                        counter(val + 1)
                    }
                }

                counter(0)
                "#
                .into(),
                big_dec!(101),
            ),
        ];

        run_vm_tests(tests);
    }

    #[test]
    fn test_closures() {
        let tests = vec![
            // 测试1: 基本闭包
            VmTestCase::<BigDecimal>::new(
                r#"
                func new_adder(a, b) {
                    func inner(c) {
                        a + b + c
                    }

                    inner
                }

                let adder = new_adder(1, 2);
                adder(8);
                "#
                .into(),
                big_dec!(11),
            ),
            // 测试2: 带局部变量的闭包
            VmTestCase::<BigDecimal>::new(
                r#"
                func new_adder(a, b) { 
                    let c = a + b; 
                    func inner(d) { c + d }; 

                    inner
                }; 
                let adder = new_adder(1, 2); 
                adder(8); 
                "#
                .into(),
                big_dec!(11),
            ),
            // 测试1: 多层闭包
            VmTestCase::<BigDecimal>::new(
                r#"
                func new_adder_outer(a, b) { 
                    let c = a + b; 
                    func inner1(d) { 
                        let e = d + c; 
                        func inner2(f) { e + f; }; 

                        inner2
                    }; 

                    inner1
                }; 
                let new_adder_inner = new_adder_outer(1, 2); 
                let adder = new_adder_inner(3); 
                adder(8); 
                "#
                .into(),
                big_dec!(14),
            ),
            // 测试3: 多层变量捕获
            VmTestCase::<BigDecimal>::new(
                r#"
                let a = 1; 
                func new_adder_outer(b) { 
                    func inner1(c) { 
                        func inner2(d) { a + b + c + d }; 

                        inner2
                    };  

                    inner1
                }; 
                let new_adder_inner = new_adder_outer(2); 
                let adder = new_adder_inner(3); 
                adder(8); 
                "#
                .into(),
                big_dec!(14),
            ),
            // 测试4: 多个闭包组合
            VmTestCase::<BigDecimal>::new(
                r#"
                func new_closure(a, b) { 
                    func param1() { a; }; 
                    func param2() { b; }; 
                    func last_func() { param1() + param2(); }; 

                    last_func
                }; 
                let closure = new_closure(9, 90); 
                closure(); 
                "#
                .into(),
                big_dec!(99),
            ),
            VmTestCase::new(
                r#"
                func tinyjoker() {
                    func lengthdown(x) {
                        if x == 0 {
                            0
                        } else {
                            lengthdown(x - 1)
                        }
                    }

                    lengthdown(101)
                }

                tinyjoker();
                "#
                .into(),
                big_dec!(0),
            ),
        ];

        run_vm_tests(tests)
    }

    fn run_vm_tests<T: Debug + Clone>(tests: Vec<VmTestCase<T>>) {
        for test_case in tests {
            let compile_result =
                compile_it(test_case.input.clone(), "__run_vm_tests__".into());

            if let Err(msg) = compile_result {
                panic!("{}", format!("compiler error: {msg}").red());
            }

            let bytecode = compile_result.expect("compiler failed!");

            let mut vm = Vm::new(bytecode);

            let result = vm.run();
            if let Err(msg) = result {
                panic!("{}", format!("vm run error: {msg}").red());
            }

            let last_popped = vm.last_popped_stack_elem();

            match test_case.expected.type_id() {
                id if id == TypeId::of::<BigDecimal>() => {
                    let result = {
                        let last_popped = last_popped
                            .clone()
                            .expect("No value popped from stack")
                            .borrow()
                            .clone();

                        if last_popped.get_type() == INT {
                            test_integer_object(
                                convert_type_use_box!(BigDecimal, test_case.expected.clone()),
                                &last_popped,
                            )
                        } else if last_popped.get_type() == DOUBLE {
                            test_double_object(
                                convert_type_use_box!(BigDecimal, test_case.expected.clone()),
                                &last_popped,
                            )
                        } else {
                            Err(format!(
                                "Expected last popped type: {:?}, got: {}",
                                [INT, DOUBLE],
                                &last_popped.get_type()
                            ))
                        }
                    };

                    if let Err(msg) = result {
                        panic!("testIntegerObject failed: {}", msg.red());
                    }
                }

                id if id == TypeId::of::<bool>() => {
                    let expected_bool = convert_type_use_box!(bool, test_case.expected.clone());

                    let result = test_boolean_object(
                        expected_bool,
                        &last_popped
                            .clone()
                            .expect("No value popped from stack")
                            .borrow()
                            .clone(),
                    );

                    if let Err(msg) = result {
                        panic!("testBooleanObject failed: {}", msg.red());
                    }
                }

                id if id == TypeId::of::<String>() => {
                    let expected_str = convert_type_use_box!(String, test_case.expected.clone());

                    let result = test_string_object(
                        expected_str,
                        &last_popped
                            .clone()
                            .expect("No value popped from stack")
                            .borrow()
                            .clone(),
                    );

                    if let Err(msg) = result {
                        panic!("testBooleanObject failed: {}", msg.red());
                    }
                }

                id if id == TypeId::of::<()>() => {
                    let result = test_none_object(
                        &last_popped
                            .clone()
                            .expect("No value popped from stack")
                            .borrow()
                            .clone(),
                    );

                    if let Err(msg) = result {
                        panic!("testNoneObject failed: {}", msg.red());
                    }
                }

                id if id == TypeId::of::<Vec<BigDecimal>>() => {
                    let vec = convert_type_use_box!(Vec<BigDecimal>, test_case.expected.clone());

                    let last_popped = last_popped
                        .clone()
                        .expect("No value popped from stack")
                        .borrow()
                        .clone();

                    if let Some(arr) = last_popped.as_any().downcast_ref::<AntArray>() {
                        if arr.items.len() != vec.len() {
                            panic!(
                                "wrong number of items. want = {}, got = {}",
                                vec.len(),
                                arr.items.len()
                            )
                        }

                        for (expected, actual) in vec.iter().zip(&arr.items) {
                            let result = {
                                if actual.get_type() == INT {
                                    test_integer_object(expected.clone(), &actual)
                                } else if actual.get_type() == DOUBLE {
                                    test_double_object(expected.clone(), &actual)
                                } else {
                                    Err(format!(
                                        "Expected item type: {:?}, got: {}",
                                        [INT, DOUBLE],
                                        &actual.get_type()
                                    ))
                                }
                            };

                            if let Err(msg) = result {
                                panic!("testIntegerObject failed: {}", msg.red());
                            }
                        }
                    } else {
                        panic!("Expected an array object, got: {:?}", last_popped)
                    }
                }

                _ => {}
            }

            println!(
                "{}: {}",
                format!("Test passed for input: {}", test_case.input).green(),
                format!("Expected: {:?}, got: {:?}", test_case.expected, last_popped).green()
            );
        }
    }

    fn test_string_object(expected: String, actual: &Object) -> Result<(), String> {
        let str_obj = try_unwrap!(actual, Object::AntString(_))
            .expect("expected an string");

        if str_obj.value != expected {
            Err(format!(
                "object has wrong value. got = {}, want = {}",
                str_obj.value, expected
            ))
        } else {
            Ok(())
        }
    }

    fn test_none_object(actual: &Object) -> Result<(), String> {
        let o = try_unwrap!(actual, Object::AntNone(_));

        if o.is_none() {
            Err(format!(
                "object is not nothing!",
            ))
        } else {
            Ok(())
        }
    }

    fn test_boolean_object(expected: bool, actual: &Object) -> Result<(), String> {
        let bool_obj = try_unwrap!(actual, Object::AntBoolean(_))
            .expect("expected an boolean");

        if bool_obj.value != expected {
            Err(format!(
                "object has wrong value. got = {}, want = {}",
                bool_obj.value, expected
            ))
        } else {
            Ok(())
        }
    }

    fn test_integer_object(expected: BigDecimal, actual: &Object) -> Result<(), String> {
        let int_obj = try_unwrap!(actual, Object::AntInt(_))
            .expect("expected an integer");

        if int_obj.value != expected {
            Err(format!(
                "object has wrong value. got = {}, want = {}",
                int_obj.value, expected
            ))
        } else {
            Ok(())
        }
    }

    fn test_double_object(expected: BigDecimal, actual: &Object) -> Result<(), String> {
        let double_obj = try_unwrap!(actual, Object::AntDouble(_))
            .expect("expected an doubke");

        if double_obj.value != expected {
            Err(format!(
                "object has wrong value. got = {}, want = {}",
                double_obj.value, expected
            ))
        } else {
            Ok(())
        }
    }
}
