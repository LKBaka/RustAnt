#[cfg(test)]
mod tests {
    use crate::byte_code_vm::code::code::{
        OP_ADD, OP_CALL, OP_CLOSURE, OP_CONSTANTS, OpCode, instruction_to_str, lookup, make,
        read_operands,
    };

    #[test]
    fn test_make() {
        struct TestCase {
            op: OpCode,
            operands: Vec<u16>,
            expected: Vec<u8>,
        }

        let tests = vec![
            TestCase {
                op: OP_CONSTANTS,
                operands: vec![65534],
                expected: vec![OP_CONSTANTS as u8, 255, 254],
            },
            TestCase {
                op: OP_CALL,
                operands: vec![0],
                expected: vec![OP_CALL as u8, 0, 0],
            },
            TestCase {
                op: OP_CLOSURE,
                operands: vec![0, 0],
                expected: vec![OP_CLOSURE as u8, 0, 0, 0, 0],
            },
            TestCase {
                op: OP_ADD,
                operands: vec![],
                expected: vec![OP_ADD as u8],
            },
            // 可添加更多测试用例...
        ];

        for tt in tests {
            let instruction = make(tt.op, &tt.operands);

            assert_eq!(
                instruction.len(),
                tt.expected.len(),
                "instruction has wrong length. want={}, got={}",
                tt.expected.len(),
                instruction.len()
            );

            for (i, &expected_byte) in tt.expected.iter().enumerate() {
                assert_eq!(
                    instruction[i], expected_byte,
                    "wrong byte at pos {}. want={}, got={}",
                    i, expected_byte, instruction[i]
                );
            }
        }
    }

    #[test]
    fn test_instructions_string() {
        let instructions = vec![
            make(OP_ADD, &vec![]),
            make(OP_CONSTANTS, &vec![1u16]),
            make(OP_CONSTANTS, &vec![2u16]),
            make(OP_CONSTANTS, &vec![65535u16]),
            make(OP_CALL, &vec![255u16]),
            make(OP_CLOSURE, &vec![0u16, 0u16]),
        ];

        let mut expected = String::new();
        expected.push_str("0000 OpAdd\n");
        expected.push_str("0001 OpConstant 1\n");
        expected.push_str("0004 OpConstant 2\n");
        expected.push_str("0007 OpConstant 65535\n");
        expected.push_str("0010 OpCall 255\n");
        expected.push_str("0013 OpClosure 0 0\n");

        let mut concatted: Vec<u8> = vec![];

        for mut ins in instructions {
            concatted.append(&mut ins);
        }

        if instruction_to_str(&concatted) != expected {
            panic!(
                "instructions wrongly formatted.\nwant = {}\ngot = {}",
                expected,
                instruction_to_str(&concatted)
            )
        }
    }

    #[test]
    fn test_read_operands() {
        struct ReadOperandsTestCase {
            op: OpCode,
            operands: Vec<u16>,
            bytes_read: i32,
        }

        impl ReadOperandsTestCase {
            pub fn new(op: OpCode, operands: Vec<u16>, bytes_read: i32) -> Self {
                Self {
                    op,
                    operands,
                    bytes_read,
                }
            }
        }

        let tests = vec![
            ReadOperandsTestCase::new(OP_CONSTANTS, vec![65535u16], 2),
            ReadOperandsTestCase::new(OP_CALL, vec![0u16], 2),
            ReadOperandsTestCase::new(OP_CLOSURE, vec![0u16, 0u16], 4),
        ];

        for test_case in tests {
            let instruction = make(test_case.op, &test_case.operands);

            let def = {
                match lookup(test_case.op) {
                    Ok(it) => it,
                    Err(msg) => {
                        panic!("definition not found: {msg}\n")
                    }
                }
            };

            let result = read_operands(&def, &instruction[1..].to_vec());

            let operands_read = result.0;
            let offset = result.1;

            if offset != test_case.bytes_read as usize {
                panic!(
                    "offset wrong. want = {}, got = {}",
                    test_case.bytes_read, offset
                )
            }

            for (i, want) in test_case.operands.iter().enumerate() {
                if operands_read[i] != *want as i32 {
                    panic!("operand wrong. want = {}, got = {}", want, operands_read[i])
                }
            }
        }
    }
}
