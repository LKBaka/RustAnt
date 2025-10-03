#[cfg(test)]
mod tests {
    use std::{
        any::{Any, TypeId}, cell::RefCell, fmt::Debug, rc::Rc
    };

    use bigdecimal::BigDecimal;
    use colored::Colorize;

    use crate::{
        big_dec, byte_code_vm::{
            code::code::{
                instruction_to_str, make, Instructions, OP_ADD, OP_ARRAY, OP_BANG, OP_CONSTANTS, OP_DIVIDE, OP_EQ, OP_FALSE, OP_GET_GLOBAL, OP_GT, OP_HASH, OP_INDEX, OP_JUMP, OP_JUMP_NOT_TRUTHY, OP_MINUS, OP_MULTIPLY, OP_NONE, OP_NOTEQ, OP_POP, OP_SET_GLOBAL, OP_SUBTRACT, OP_TRUE
            },
            compiler::compiler::Compiler, scope_info::ScopeInfo,
        }, convert_type_use_box, obj_enum::object::Object, object::{ant_int::AntInt, ant_string::AntString}, parser::utils::parse
    };

    struct CompilerTestCase<T: Debug + Clone + 'static> {
        input: String,
        expected_constants: Vec<T>,
        expected_instructions: Vec<Instructions>,
    }

    impl<T: Debug + Clone> CompilerTestCase<T> {
        pub fn new(
            input: String,
            expected_constants: Vec<T>,
            expected_instructions: Vec<Instructions>,
        ) -> Self {
            Self {
                input,
                expected_constants,
                expected_instructions,
            }
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
                ],
            ),
            CompilerTestCase::new(
                "1 + 2".into(),
                vec![Box::new(big_dec!(1)), Box::new(big_dec!(2))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_ADD, &vec![]),
                    make(OP_POP, &vec![]),
                ],
            ),
            CompilerTestCase::new(
                "1 - 2".into(),
                vec![Box::new(big_dec!(1)), Box::new(big_dec!(2))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_SUBTRACT, &vec![]),
                    make(OP_POP, &vec![]),
                ],
            ),
            CompilerTestCase::new(
                "1 * 2".into(),
                vec![Box::new(big_dec!(1)), Box::new(big_dec!(2))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_MULTIPLY, &vec![]),
                    make(OP_POP, &vec![]),
                ],
            ),
            CompilerTestCase::new(
                "1 / 2".into(),
                vec![Box::new(big_dec!(1)), Box::new(big_dec!(2))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_DIVIDE, &vec![]),
                    make(OP_POP, &vec![]),
                ],
            ),
            CompilerTestCase::<Box<BigDecimal>>::new(
                "-1".into(),
                vec![Box::new(big_dec!(1))],
                vec![
                    make(OP_CONSTANTS, &vec![]), 
                    make(OP_MINUS, &vec![]),
                    make(OP_POP, &vec![]),
                ],
            ),
        ];

        run_compiler_tests(tests)
    }

    #[test]
    fn test_conditionals() {
        let tests = vec![
            CompilerTestCase::new(
                "if true {10}; 42".into(),
                vec![Box::new(big_dec!(10)), Box::new(big_dec!(42))],
                vec![
                    // 0000
                    make(OP_TRUE, &vec![]),
                    // 0001
                    make(OP_JUMP_NOT_TRUTHY, &vec![7u16]),
                    // 0004
                    make(OP_CONSTANTS, &vec![0u16]),
                    // 0007
                    make(OP_NONE, &vec![]),
                    // 0008
                    make(OP_POP, &vec![]),
                    // 0009
                    make(OP_CONSTANTS, &vec![1u16]),
                    // 0012
                    make(OP_POP, &vec![]),
                ],
            ),
            CompilerTestCase::new(
                "if true { 10 } else { 20 }; 3333;".into(),
                vec![
                    Box::new(big_dec!(10)),
                    Box::new(big_dec!(20)),
                    Box::new(big_dec!(3333)),
                ],
                vec![
                    // 0000
                    make(OP_TRUE, &vec![]),
                    // 0001
                    make(OP_JUMP_NOT_TRUTHY, &vec![10]),
                    // 0004
                    make(OP_CONSTANTS, &vec![0]),
                    // 0007
                    make(OP_JUMP, &vec![13]),
                    // 0010
                    make(OP_CONSTANTS, &vec![1]),
                    // 0013
                    make(OP_POP, &vec![]),
                    // 0014
                    make(OP_CONSTANTS, &vec![2]),
                    // 0017
                    make(OP_POP, &vec![]),
                ],
            ),
            CompilerTestCase::new(
                "if true { 10 } else if false {20} else { 30 }; 3333;".into(),
                vec![
                    Box::new(big_dec!(10)),
                    Box::new(big_dec!(20)),
                    Box::new(big_dec!(30)),
                    Box::new(big_dec!(3333)),
                ],
                vec![
                    // 0000
                    make(OP_TRUE, &vec![]), // if condition
                    // 0001
                    make(OP_JUMP_NOT_TRUTHY, &vec![10]), // if
                    // 0004
                    make(OP_CONSTANTS, &vec![0]), // if body
                    // 0007
                    make(OP_JUMP, &vec![23]), // goto end
                    // 0010
                    make(OP_FALSE, &vec![]), // else if condition
                    // 0011
                    make(OP_JUMP_NOT_TRUTHY, &vec![20]), // else if
                    // 0014
                    make(OP_CONSTANTS, &vec![1]), // else if body
                    // 0017
                    make(OP_JUMP, &vec![23]), // goto end
                    // 0020
                    make(OP_CONSTANTS, &vec![2]), // else body
                    // 0023
                    // if expression all end
                    make(OP_POP, &vec![]),
                    // 0024
                    make(OP_CONSTANTS, &vec![3]), // 3333
                    // 0027
                    make(OP_POP, &vec![]), // pop 3333
                ],
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
                vec![make(OP_TRUE, &vec![]), make(OP_POP, &vec![])],
            ),
            CompilerTestCase::<Box<BigDecimal>>::new(
                "false".into(),
                vec![],
                vec![make(OP_FALSE, &vec![]), make(OP_POP, &vec![])],
            ),
            CompilerTestCase::new(
                "1 > 2".into(),
                vec![Box::new(big_dec!(1)), Box::new(big_dec!(2))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_GT, &vec![]),
                    make(OP_POP, &vec![]),
                ],
            ),
            CompilerTestCase::new(
                "1 < 2".into(),
                vec![Box::new(big_dec!(2)), Box::new(big_dec!(1))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_GT, &vec![]),
                    make(OP_POP, &vec![]),
                ],
            ),
            CompilerTestCase::new(
                "1 == 2".into(),
                vec![Box::new(big_dec!(2)), Box::new(big_dec!(1))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_EQ, &vec![]),
                    make(OP_POP, &vec![]),
                ],
            ),
            CompilerTestCase::new(
                "1 !=  2".into(),
                vec![Box::new(big_dec!(2)), Box::new(big_dec!(1))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_NOTEQ, &vec![]),
                    make(OP_POP, &vec![]),
                ],
            ),
            CompilerTestCase::<Box<BigDecimal>>::new(
                "true == false".into(),
                vec![],
                vec![
                    make(OP_TRUE, &vec![]),
                    make(OP_FALSE, &vec![]),
                    make(OP_EQ, &vec![]),
                    make(OP_POP, &vec![]),
                ],
            ),
            CompilerTestCase::<Box<BigDecimal>>::new(
                "true != false".into(),
                vec![],
                vec![
                    make(OP_TRUE, &vec![]),
                    make(OP_FALSE, &vec![]),
                    make(OP_NOTEQ, &vec![]),
                    make(OP_POP, &vec![]),
                ],
            ),
            CompilerTestCase::<Box<BigDecimal>>::new(
                "!true".into(),
                vec![],
                vec![make(OP_TRUE, &vec![]), make(OP_BANG, &vec![]), make(OP_POP, &vec![])],
            ),
        ];

        run_compiler_tests(tests)
    }

    #[test]
    fn test_none_literal() {
        let tests: Vec<CompilerTestCase<()>> = vec![
            CompilerTestCase::new(
                "none; nOne; NONE; None".into(),
                vec![],
                vec![
                    make(OP_NONE, &vec![]),
                    make(OP_POP, &vec![]),
                    make(OP_NONE, &vec![]),
                    make(OP_POP, &vec![]),
                    make(OP_NONE, &vec![]),
                    make(OP_POP, &vec![]),
                    make(OP_NONE, &vec![]),
                    make(OP_POP, &vec![]),
                ],
            ),
        ];

        run_compiler_tests(tests)
    }

    #[test]
    fn test_hash_literal() {
        let tests: Vec<CompilerTestCase<Box<BigDecimal>>> = vec![
            CompilerTestCase::new(
                "{}".into(),
                vec![],
                vec![
                    make(OP_HASH, &vec![0]),
                    make(OP_POP, &vec![]),
                ],
            ),
            CompilerTestCase::new(
                "{1: 2, 3: 4, 5: 6}".into(),
                vec![
                    Box::new(big_dec!(1)), Box::new(big_dec!(2)),
                    Box::new(big_dec!(3)), Box::new(big_dec!(4)),
                    Box::new(big_dec!(5)), Box::new(big_dec!(6))
                ],
                vec![
                    make(OP_CONSTANTS, &vec![0]),
                    make(OP_CONSTANTS, &vec![1]),
                    make(OP_CONSTANTS, &vec![2]),
                    make(OP_CONSTANTS, &vec![3]),
                    make(OP_CONSTANTS, &vec![4]),
                    make(OP_CONSTANTS, &vec![5]),
                    make(OP_HASH, &vec![6]),
                    make(OP_POP, &vec![]),
                ],
            ),
            CompilerTestCase::new(
                "{1: 2 + 3, 4: 5 * 6}".into(),
                vec![
                    Box::new(big_dec!(1)), Box::new(big_dec!(2)),
                    Box::new(big_dec!(3)), Box::new(big_dec!(4)),
                    Box::new(big_dec!(5)), Box::new(big_dec!(6))
                ],
                vec![
                    make(OP_CONSTANTS, &vec![0]),
                    make(OP_CONSTANTS, &vec![1]),
                    make(OP_CONSTANTS, &vec![2]),
                    make(OP_ADD, &vec![]),
                    make(OP_CONSTANTS, &vec![3]),
                    make(OP_CONSTANTS, &vec![4]),
                    make(OP_CONSTANTS, &vec![5]),
                    make(OP_MULTIPLY, &vec![]),
                    make(OP_HASH, &vec![4]),
                    make(OP_POP, &vec![]),
                ],
            ),
        ];

        run_compiler_tests(tests)
    }

    #[test]
    fn test_global_let_statements() {
        let tests = vec![
            // 测试1: 两个全局变量声明
            CompilerTestCase::new(
                r#"
                let one = 1;
                let two = 2;
                "#
                .into(),
                vec![Box::new(big_dec!(1)), Box::new(big_dec!(2))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_SET_GLOBAL, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_SET_GLOBAL, &vec![1u16]),
                ],
            ),
            // 测试2: 声明后使用变量
            CompilerTestCase::new(
                r#"
                let one = 1;
                one;
                "#
                .into(),
                vec![Box::new(big_dec!(1))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_SET_GLOBAL, &vec![0u16]),
                    make(OP_GET_GLOBAL, &vec![0u16]),
                    make(OP_POP, &vec![]),
                ],
            ),
            // 测试3: 变量赋值给另一个变量
            CompilerTestCase::new(
                r#"
                let one = 1;
                let two = one;
                two;
                "#
                .into(),
                vec![Box::new(big_dec!(1))],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_SET_GLOBAL, &vec![0u16]),
                    make(OP_GET_GLOBAL, &vec![0u16]),
                    make(OP_SET_GLOBAL, &vec![1u16]),
                    make(OP_GET_GLOBAL, &vec![1u16]),
                    make(OP_POP, &vec![]),
                ],
            ),
        ];

        run_compiler_tests(tests)
    }

    #[test]
    fn test_string_expressions() {
        let tests = vec![
            // 测试1: 简单字符串字面量
            CompilerTestCase::new(
                "\"lava\"".into(),
                vec![Box::new("lava".to_string())],
                vec![make(OP_CONSTANTS, &vec![0u16]), make(OP_POP, &vec![]),],
            ),
            // 测试2: 字符串拼接
            CompilerTestCase::new(
                "\"la\" + \"va\"".into(),
                vec![Box::new("la".to_string()), Box::new("va".to_string())],
                vec![
                    make(OP_CONSTANTS, &vec![0u16]),
                    make(OP_CONSTANTS, &vec![1u16]),
                    make(OP_ADD, &vec![]),
                    make(OP_POP, &vec![]),
                ],
            ),
        ];

        run_compiler_tests(tests)
    }

    #[test]
    fn test_array_literal() {
        let tests = vec![CompilerTestCase::new(
            "[1, 2, 3]".into(),
            vec![
                Box::new(big_dec!(1)),
                Box::new(big_dec!(2)),
                Box::new(big_dec!(3)),
            ],
            vec![
                make(OP_CONSTANTS, &vec![0u16]),
                make(OP_CONSTANTS, &vec![1u16]),
                make(OP_CONSTANTS, &vec![2u16]),
                make(OP_ARRAY, &vec![3u16]),
                make(OP_POP, &vec![]),
            ],
        )];

        run_compiler_tests(tests);
    }

    #[test]
    fn test_index_expressions() {
        let tests = vec![CompilerTestCase::new(
            "[1, 2, 3][1 + 1]".into(),
            vec![
                Box::new(big_dec!(1)),
                Box::new(big_dec!(2)),
                Box::new(big_dec!(3)),
                Box::new(big_dec!(1)),
                Box::new(big_dec!(1)),
            ],
            vec![
                make(OP_CONSTANTS, &vec![0u16]),
                make(OP_CONSTANTS, &vec![1u16]),
                make(OP_CONSTANTS, &vec![2u16]),
                make(OP_ARRAY, &vec![3u16]),
                make(OP_CONSTANTS, &vec![3u16]),
                make(OP_CONSTANTS, &vec![4u16]),
                make(OP_ADD, &vec![]),
                make(OP_INDEX, &vec![]),
                make(OP_POP, &vec![]),
            ],
        )];

        run_compiler_tests(tests);
    }

    fn run_compiler_tests<T: Debug + Clone>(tests: Vec<CompilerTestCase<T>>) {
        for test_case in tests {
            let program = parse(test_case.input, "__run_compiler_tests__".into());
            if let Err(_) = program {
                panic!("parser failed!")
            }

            let mut compiler = Compiler::new("__run_compiler_tests__".into());
            if let Err(msg) = compiler.start_compile(program.expect("parser failed!")) {
                panic!("compiler error: {msg}")
            }

            let bytecode = compiler.bytecode();
            if let Err(msg) =
                test_instructions(test_case.expected_instructions, bytecode.instructions)
            {
                panic!("testInstructions failed: {msg}")
            }

            if let Err(msg) = test_constants(test_case.expected_constants, bytecode.constants) {
                panic!("testConstants failed: {msg}")
            }
        }
    }

    fn test_instructions(expected: Vec<Instructions>, actual: Instructions) -> Result<(), String> {
        let concatted = concat_instructions(expected);

        if actual.len() != concatted.len() {
            return Err(format!(
                "wrong instructions length.\nwant = {}\ngot = {}",
                instruction_to_str(&concatted),
                instruction_to_str(&actual)
            ));
        }

        for (i, ins) in concatted.iter().enumerate() {
            if actual[i] != *ins {
                return Err(format!(
                    "wrong instruction at {i}.\nwant = {}\ngot = {}",
                    instruction_to_str(&concatted),
                    instruction_to_str(&actual)
                ));
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
        actual: Vec<Rc<RefCell<Object>>>,
    ) -> Result<(), String> {
        if expected.len() != actual.len() {
            return Err(format!(
                "wrong number of constants. want = {}, got = {}",
                expected.len(),
                actual.len()
            ));
        }

        for (i, constant) in expected.iter().enumerate() {
            match constant.type_id() {
                id if id == TypeId::of::<BigDecimal>() => {
                    let result = test_integer_object(
                        convert_type_use_box!(BigDecimal, constant.clone()),
                        actual[i].borrow().clone(),
                    );

                    if let Err(msg) = result {
                        return Err(format!(
                            "constant {} - testIntegerObject failed: {}",
                            i, msg
                        ));
                    }
                }

                id if id == TypeId::of::<String>() => {
                    let result = test_string_object(
                        convert_type_use_box!(String, constant.clone()),
                        actual[i].borrow().clone(),
                    );

                    if let Err(msg) = result {
                        return Err(format!("constant {} - testStringObject failed: {}", i, msg));
                    }
                }

                TypeId { .. } => {}
            }
        }

        return Ok(());
    }

    fn test_integer_object(expected: BigDecimal, actual: Object) -> Result<(), String> {
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

    fn test_string_object(expected: String, actual: Object) -> Result<(), String> {
        let str_obj = convert_type_use_box!(AntString, actual.clone());

        if str_obj.value != expected {
            Err(format!(
                "object has wrong value. got = {}, want = {}",
                str_obj.value, expected
            ))
        } else {
            Ok(())
        }
    }

    fn scope_index_wrong_err_print(want: usize, got: usize) {
        panic!(
            "{}",
            format!("scopeIndex wrong. want: {want}, got: {got}").red()
        )
    }

    #[test]
    fn test_compiler_scopes() {
        let mut compiler = Compiler::new("__run_compiler_tests__".into());

        let global_symbol_table = compiler.symbol_table.clone();

        if compiler.scope_index != 0 {
            scope_index_wrong_err_print(0, compiler.scope_index);
        }

        compiler.emit(OP_MULTIPLY, vec![]);

        compiler.enter_scope(ScopeInfo {
            file_name: "__run_compiler_tests__".into(),
            scope_name: "__run_compiler_tests__".into()
        });

        if compiler.scope_index != 1 {
            scope_index_wrong_err_print(1, compiler.scope_index);
        }

        compiler.emit(OP_SUBTRACT, vec![]);

        if compiler.scopes[compiler.scope_index]
            .instructions
            .borrow()
            .len()
            != 1
        {
            panic!(
                "{}",
                format!(
                    "instructions length wrong, got: {}",
                    compiler.scopes[compiler.scope_index]
                        .instructions
                        .borrow()
                        .len()
                )
                .red()
            )
        }

        let last = compiler.scopes[compiler.scope_index].last_instruction;

        if last.op != OP_SUBTRACT {
            panic!(
                "{}",
                format!(
                    "lastInstruction opcode wrong. want: {}, got: {}",
                    OP_SUBTRACT, last.op
                )
                .red()
            )
        }

        if compiler.symbol_table.borrow().outer != Some(global_symbol_table) {
            panic!("{}", "compiler did not enclose symbol table")
        }

        compiler.leave_scope();

        if compiler.scope_index != 0 {
            scope_index_wrong_err_print(0, compiler.scope_index);
        }

        compiler.emit(OP_ADD, vec![]);

        if compiler.scopes[compiler.scope_index]
            .instructions
            .borrow()
            .len()
            != 2
        {
            panic!(
                "{}",
                format!(
                    "instructions length wrong, got: {}",
                    compiler.scopes[compiler.scope_index]
                        .instructions
                        .borrow()
                        .len()
                )
                .red()
            )
        }

        let last = compiler.scopes[compiler.scope_index].last_instruction;

        if last.op != OP_ADD {
            panic!(
                "{}",
                format!(
                    "lastInstruction opcode wrong. want: {}, got: {}",
                    OP_ADD, last.op
                )
                .red()
            )
        }

        let previous = compiler.scopes[compiler.scope_index].previous_instruction;
        if previous.op != OP_MULTIPLY {
            panic!(
                "{}",
                format!(
                    "previousInstruction opcode wrong. want: {}, got: {}",
                    OP_MULTIPLY, last.op
                )
                .red()
            )
        }
    }
}
