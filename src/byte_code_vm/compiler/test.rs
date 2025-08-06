#[cfg(test)]
mod tests {
    use std::{any::{Any, TypeId}, fmt::Debug};

    use bigdecimal::BigDecimal;

    use crate::{big_dec, byte_code_vm::{code::code::{instruction_to_str, make, Instructions, OP_ADD, OP_BANG, OP_CONSTANTS, OP_DIVIDE, OP_EQ, OP_FALSE, OP_GT, OP_MINUS, OP_MULTIPLY, OP_NOTEQ, OP_POP, OP_SUBTRACT, OP_TRUE}, compiler::compiler::Compiler}, convert_type_use_box, object::{ant_int::AntInt, object::Object}, parser::utils::parse};

    struct CompilerTestCase<T: Debug + Clone + 'static> {
        input: String,
        expected_constants: Vec<T>,
        expected_instructions: Vec<Instructions>
    }

    impl<T: Debug + Clone> CompilerTestCase<T> {
        pub fn new(
            input: String,
            expected_constants: Vec<T>,
            expected_instructions: Vec<Instructions>
        ) -> Self {
            Self { input, expected_constants, expected_instructions }
        }
    }

    #[test]
    fn test_integer_arithmetic() {
        let tests = vec![
            CompilerTestCase::new(
                "1; 2".into(),
                vec![Box::new(big_dec!(1)), Box::new(big_dec!(2))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_POP, &vec![]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_POP, &vec![]),
                ]
            ),
            CompilerTestCase::new(
                "1 + 2".into(),
                vec![Box::new(big_dec!(1)), Box::new(big_dec!(2))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_ADD, &vec![]),
                    make(OP_POP, &vec![]),
                ]
            ),
            CompilerTestCase::new(
                "1 - 2".into(),
                vec![Box::new(big_dec!(1)), Box::new(big_dec!(2))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_SUBTRACT, &vec![]),
                    make(OP_POP, &vec![]),
                ]
            ),
            CompilerTestCase::new(
                "1 * 2".into(),
                vec![Box::new(big_dec!(1)), Box::new(big_dec!(2))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_MULTIPLY, &vec![]),
                    make(OP_POP, &vec![]),
                ]
            ),
            CompilerTestCase::new(
                "1 / 2".into(),
                vec![Box::new(big_dec!(1)), Box::new(big_dec!(2))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_DIVIDE, &vec![]),
                    make(OP_POP, &vec![]),
                ]
            ),
            CompilerTestCase::<Box<BigDecimal>>::new(
                "-1".into(),
                vec![Box::new(big_dec!(1))],
                vec![
                    make(OP_CONSTANTS, &vec![]),
                    make(OP_MINUS, &vec![]),
                    make(OP_POP, &vec![]),
                ]
            ),
        ];

        run_compiler_tests(tests)
    }

    
    #[test]
    fn test_boolean_expressions() {
        let tests = vec![
            CompilerTestCase::<Box<BigDecimal>>::new(
                "true".into(),
                vec![],
                vec![
                    make(OP_TRUE, &vec![]),
                    make(OP_POP, &vec![]),
                ]
            ),
            CompilerTestCase::<Box<BigDecimal>>::new(
                "false".into(),
                vec![],
                vec![
                    make(OP_FALSE, &vec![]),
                    make(OP_POP, &vec![]),
                ]
            ),
            CompilerTestCase::new(
                "1 > 2".into(),
                vec![Box::new(big_dec!(1)), Box::new(big_dec!(2))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_GT, &vec![]),
                    make(OP_POP, &vec![]),
                ]
            ),
            CompilerTestCase::new(
                "1 < 2".into(),
                vec![Box::new(big_dec!(2)), Box::new(big_dec!(1))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_GT, &vec![]),
                    make(OP_POP, &vec![]),
                ]
            ),
            CompilerTestCase::new(
                "1 == 2".into(),
                vec![Box::new(big_dec!(2)), Box::new(big_dec!(1))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_EQ, &vec![]),
                    make(OP_POP, &vec![]),
                ]
            ),
            CompilerTestCase::new(
                "1 !=  2".into(),
                vec![Box::new(big_dec!(2)), Box::new(big_dec!(1))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_NOTEQ, &vec![]),
                    make(OP_POP, &vec![]),
                ]
            ),
            CompilerTestCase::<Box<BigDecimal>>::new(
                "true == false".into(),
                vec![],
                vec![
                    make(OP_TRUE, &vec![]),
                    make(OP_FALSE, &vec![]),
                    make(OP_EQ, &vec![]),
                    make(OP_POP, &vec![]),
                ]
            ),
            CompilerTestCase::<Box<BigDecimal>>::new(
                "true != false".into(),
                vec![],
                vec![
                    make(OP_TRUE, &vec![]),
                    make(OP_FALSE, &vec![]),
                    make(OP_NOTEQ, &vec![]),
                    make(OP_POP, &vec![]),
                ]
            ),
            CompilerTestCase::<Box<BigDecimal>>::new(
                "!true".into(),
                vec![],
                vec![
                    make(OP_TRUE, &vec![]),
                    make(OP_BANG, &vec![]),
                    make(OP_POP, &vec![]),
                ]
            ),
        ];

        run_compiler_tests(tests)
    }


    fn run_compiler_tests<T: Debug + Clone>(tests: Vec<CompilerTestCase<T>>) {
        for test_case in tests {
            let program = parse(test_case.input, "__run_compiler_tests__".into());
            if let Err(_) = program {
                panic!("parser failed!")
            }

            let mut compiler = Compiler::new();
            if let Err(msg) = compiler.start_compile(program.expect("parser failed!")) {
                panic!("compiler error: {msg}")
            }

            let bytecode = compiler.bytecode();
            if let Err(msg) = test_instructions(test_case.expected_instructions, bytecode.instructions) {
                panic!("testInstructions failed: {msg}")
            }

            if let Err(msg) = test_constants(test_case.expected_constants, bytecode.constants) {
                panic!("testConstants failed: {msg}")
            }
        }
    }


    fn test_instructions(
        expected: Vec<Instructions>,
        actual: Instructions
    ) -> Result<(), String> {
        let concatted = concat_instructions(expected);
        
        if actual.len() != concatted.len() {
            return Err(format!(
                "wrong instructions length.\nwant = {}\ngot = {}", 
                instruction_to_str(&concatted), instruction_to_str(&actual)
            ))
        }

        for (i, ins) in concatted.iter().enumerate() {
            if actual[i] != *ins {
                return Err(format!(
                    "wrong instruction at {i}.\nwant = {}\ngot = {}", 
                    instruction_to_str(&concatted), instruction_to_str(&actual)
                ))
            }
        }

        Ok(())
    }

    fn concat_instructions(s: Vec<Vec<u8>>) -> Vec<u8> {
        let total_len: usize = s.iter().map(Vec::len).sum();

        let mut out = Vec::with_capacity(total_len);
        
        for ins in s {
            out.extend(ins);
        }
        
        out
    }

    fn test_constants<T: Debug + Clone + 'static>(
        expected: Vec<T>,
        actual: Vec<Object>
    ) -> Result<(), String> {
        if expected.len() != actual.len() {
            return Err(format!("wrong number of constants. want = {}, got = {}", expected.len(), actual.len()))
        }

        for (i, constant) in expected.iter().enumerate() {
            match constant.type_id() {
                id if id == TypeId::of::<BigDecimal>() => {
                    let result = test_integer_object(
                        convert_type_use_box!(BigDecimal, constant.clone()), actual[i].clone()
                    );

                    if let Err(msg) = result {
                        return Err(format!("constant {} - testIntegerObject failed: {}", i, msg))
                    }
                }

                _ => {}
            }
        }

        return Ok(())
    }

    fn test_integer_object(expected: BigDecimal, actual: Object) -> Result<(), String>  {
        let int_obj = convert_type_use_box!(AntInt, actual.clone());

        if int_obj.value != expected {
            Err(format!(
                "object has wrong value. got = {}, want = {}", 
                int_obj.value, expected
            ))
        } else {
            Ok(())
        }
    }
}