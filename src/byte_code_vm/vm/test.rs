#[cfg(test)]
mod tests {
    use std::{any::{Any, TypeId}, fmt::Debug, vec};

    use bigdecimal::BigDecimal;
    use colored::Colorize;

    use crate::{big_dec, byte_code_vm::{compiler::utils::compile_it, vm::vm::Vm}, convert_type, convert_type_use_box, object::{ant_boolean::AntBoolean, ant_double::AntDouble, ant_int::AntInt, object::{Object, DOUBLE, INT}}};

    struct VmTestCase<T: Debug + Clone + 'static> {
        input: String,
        expected: T,
    }

    impl<T: Debug + Clone> VmTestCase<T> {
        pub fn new(
            input: String,
            expected: T,
        ) -> Self {
            Self { input, expected }
        }
    }

    #[test]
    fn test_integer_arithmetic() {
        let tests = vec![
            VmTestCase::<BigDecimal>::new(
                "1".into(),
                big_dec!(1)
            ),
            VmTestCase::<BigDecimal>::new(
                "2".into(),
                big_dec!(2)
            ),
            VmTestCase::<BigDecimal>::new(
                "1 + 2".into(),
                big_dec!(3)
            ),
            VmTestCase::<BigDecimal>::new(
                "1 - 2".into(),
                big_dec!(-1)
            ),
            VmTestCase::<BigDecimal>::new(
                "1 * 2".into(),
                big_dec!(2)
            ),
            VmTestCase::<BigDecimal>::new(
                "4 / 2".into(),
                big_dec!(2)
            ),
            VmTestCase::<BigDecimal>::new(
                "50 / 2 * 2 + 10 - 5".into(),
                big_dec!(55)
            ),
            VmTestCase::<BigDecimal>::new(
                "5 + 5 + 5 + 5 - 10".into(),
                big_dec!(10)
            ),
            VmTestCase::<BigDecimal>::new(
                "50 / 2 * 2 + 10 - 5".into(),
                big_dec!(55)
            ),
            VmTestCase::<BigDecimal>::new(
                "2 * 2 * 2 * 2 * 2".into(),
                big_dec!(32)
            ),
            VmTestCase::<BigDecimal>::new(
                "5 * 2 + 10".into(),
                big_dec!(20)
            ),
            VmTestCase::<BigDecimal>::new(
                "5 + 2 * 10".into(),
                big_dec!(25)
            ),
            VmTestCase::<BigDecimal>::new(
                "5 * (2 + 10)".into(),
                big_dec!(60)
            ),
            VmTestCase::<BigDecimal>::new(
                "-5".into(),
                big_dec!(-5)
            ),
            VmTestCase::<BigDecimal>::new(
                "-10".into(),
                big_dec!(-10)
            ),
            VmTestCase::<BigDecimal>::new(
                "-50 + 100 + -50".into(),
                big_dec!(0)
            ),
            VmTestCase::<BigDecimal>::new(
                "(5 + 10 * 2 + 15 / 3) * 2 + -10".into(),
                big_dec!(50)
            ),
            VmTestCase::<BigDecimal>::new(
                "5 * (2 + 10)".into(),
                big_dec!(60)
            ),
        ];

        run_vm_tests::<BigDecimal>(tests)
    }

    #[test]
    fn test_boolean_expressions() {
        let tests = vec![
            VmTestCase::<bool>::new(
                "true".into(),
                true
            ),
            VmTestCase::<bool>::new(
                "false".into(),
                false
            ),
            VmTestCase::<bool>::new(
                "1 < 2".into(),
                true
            ),
            VmTestCase::<bool>::new(
                "1 > 2".into(),
                false
            ),
            VmTestCase::<bool>::new(
                "1 < 1".into(),
                false
            ),
            VmTestCase::<bool>::new(
                "1 > 1".into(),
                false
            ),
            VmTestCase::<bool>::new(
                "1 == 1".into(),
                true
            ),
            VmTestCase::<bool>::new(
                "1 != 1".into(),
                false
            ),
            VmTestCase::<bool>::new(
                "1 == 2".into(),
                false
            ),
            VmTestCase::<bool>::new(
                "1 != 2".into(),
                true
            ),
            VmTestCase::<bool>::new(
                "true == true".into(),
                true
            ),
            VmTestCase::<bool>::new(
                "false == false".into(),
                true
            ),
            VmTestCase::<bool>::new(
                "true == false".into(),
                false
            ),
            VmTestCase::<bool>::new(
                "true != false".into(),
                true
            ),
            VmTestCase::<bool>::new(
                "false != true".into(),
                true
            ),
            VmTestCase::<bool>::new(
                "(1 < 2) == true".into(),
                true
            ),
            VmTestCase::<bool>::new(
                "(1 < 2) == false".into(),
                false
            ),
            VmTestCase::<bool>::new(
                "(1 > 2) == true".into(),
                false
            ),
            VmTestCase::<bool>::new(
                "(1 > 2) == false".into(),
                true
            ),
            VmTestCase::<bool>::new(
                "!true".into(),
                false
            ),
            VmTestCase::<bool>::new(
                "!false".into(),
                true
            ),
            VmTestCase::<bool>::new(
                "!5".into(),
                false
            ),
            VmTestCase::<bool>::new(
                "!!true".into(),
                true
            ),
            VmTestCase::<bool>::new(
                "!!false".into(),
                false
            ),
            VmTestCase::<bool>::new(
                "!!5".into(),
                true
            ),
        ];

        run_vm_tests(tests);
    }

    fn run_vm_tests<T: Debug + Clone>(tests: Vec<VmTestCase<T>>) {
        for test_case in tests {
            let compile_result = 
                compile_it(test_case.input.clone(), "__run_compiler_tests__".into());
        
            if let Err(msg) = compile_result {
                panic!("{}", format!("compiler error: {msg}").red());
            }

            let mut vm = Vm::new(compile_result.expect("compiler failed!"));

            let result = vm.run();
            if let Err(msg) = result {
                panic!("{}", format!("vm run error: {msg}").red());
            }

            let last_popped = vm.last_popped_stack_elem();

            match test_case.expected.type_id() {
                id if id == TypeId::of::<BigDecimal>() => {
                    let result = {
                        let last_popped = last_popped.clone().expect("No value popped from stack");
                        if last_popped.get_type() == INT {
                            test_integer_object(
                                convert_type_use_box!(BigDecimal, test_case.expected.clone()), 
                                &last_popped
                            )
                        } else if last_popped.get_type() == DOUBLE {
                            test_double_object(
                                convert_type_use_box!(BigDecimal, test_case.expected.clone()), 
                                &last_popped
                            )
                        } else {
                            Err(format!(
                                "Expected last popped type: {:?}, got: {}",
                                [INT, DOUBLE], &last_popped.get_type()
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
                        expected_bool, &last_popped.clone().expect("No value popped from stack")
                    );

                    if let Err(msg) = result {
                        panic!("testBooleanObject failed: {}", msg.red());
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

    fn test_boolean_object(expected: bool, actual: &Object) -> Result<(), String> {
        let bool_obj = convert_type!(AntBoolean, actual);

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
        let int_obj = convert_type!(AntInt, actual);

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
        let double_obj = convert_type!(AntDouble, actual);

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