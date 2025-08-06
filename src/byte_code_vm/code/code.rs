use std::collections::HashMap;

use byteorder::{BigEndian, ByteOrder};
use colored::Colorize;
use lazy_static::lazy_static;

pub type Instructions = Vec<u8>;

pub type OpCode = u8;

pub const OP_CONSTANTS: u8 = 0;
pub const OP_ADD: u8 = 1;
pub const OP_SUBTRACT: u8 = 2;
pub const OP_MULTIPLY: u8 = 3;
pub const OP_DIVIDE: u8 = 4;
pub const OP_GT: u8 = 6;
pub const OP_EQ: u8 = 7;
pub const OP_NOTEQ: u8 = 8;
pub const OP_POP: u8 = 9;
pub const OP_TRUE: u8 = 10;
pub const OP_FALSE: u8 = 11;
pub const OP_MINUS: u8 = 12;
pub const OP_BANG: u8 = 13;

pub const INFIX_OPERATOR_TO_OPCODE: phf::Map<&'static str, OpCode> = phf::phf_map! {
    "+" => OP_ADD,
    "-" => OP_SUBTRACT,
    "*" => OP_MULTIPLY,
    "/" => OP_DIVIDE,
    ">" => OP_GT,
    "==" => OP_EQ,
    "!=" => OP_NOTEQ,
};

pub const PREFIX_OPERATOR_TO_OPCODE: phf::Map<&'static str, OpCode> = phf::phf_map! {
    "-" => OP_MINUS,
    "!" => OP_BANG,
};


#[derive(Debug, Clone)]
pub struct Definition {
    pub name: String,
    pub operand_widths: Vec<i32>
}

impl Definition {
    pub fn new(name: String, operand_widths: Vec<i32>) -> Self {
        Definition {
            name,
            operand_widths
        }
    }
}

lazy_static! {
    pub static ref definitions: HashMap<OpCode, Definition> = {
        let mut m = HashMap::new();

        m.insert(OP_CONSTANTS, Definition::new("OpConstant".into(), vec![2]));
        m.insert(OP_ADD, Definition::new("OpAdd".into(), vec![]));
        m.insert(OP_SUBTRACT, Definition::new("OpSubtract".into(), vec![]));
        m.insert(OP_MULTIPLY, Definition::new("OpMultiply".into(), vec![]));
        m.insert(OP_DIVIDE, Definition::new("OpDivide".into(), vec![]));
        m.insert(OP_GT, Definition::new("OpGreaterThan".into(), vec![]));
        m.insert(OP_EQ, Definition::new("OpEqual".into(), vec![]));
        m.insert(OP_NOTEQ, Definition::new("OpNotEqual".into(), vec![]));
        m.insert(OP_POP, Definition::new("OpPop".into(), vec![]));
        m.insert(OP_TRUE, Definition::new("OpTrue".into(), vec![]));
        m.insert(OP_FALSE, Definition::new("OpFalse".into(), vec![]));
        m.insert(OP_MINUS, Definition::new("OpMinus".into(), vec![]));
        m.insert(OP_BANG, Definition::new("OpBang".into(), vec![]));

        m
    };
}

pub fn lookup(op: u8) -> Result<Definition, String> {
    let definition = definitions.get(&op);

    match definition {
        Some(it) => Ok(it.clone()),
        None => Err(format!("opcode {op} undefined"))
    }
}


pub fn make(op: OpCode, operands: &Vec<u16>) -> Vec<u8> {
    let definition = definitions.get(&op);

    match definition {
        Some(def) => {
            let instruction_len = 1 + def.operand_widths.iter().sum::<i32>();

            let mut instruction = vec![0u8; instruction_len as usize];
            instruction[0] = op;

            let mut offset = 1;

            for (i, operand) in operands.iter().enumerate() {
                let width = def.operand_widths[i];
                
                match width {
                    2 => {
                        // 处理 2 字节操作数 (大端序)
                        let bytes = operand.to_be_bytes();
                        instruction[offset..offset + 2].copy_from_slice(&bytes);
                    }

                    // 添加其他宽度处理...
                    _ => panic!("unsupported operand width: {}", width), 
                }

                offset += width as usize;
            }            

            instruction
        },
        None => vec![]
    }
}

pub fn instruction_to_str(ins: &Instructions) -> String {
    let mut s = String::new();

    let mut i = 0;

    let ins_length = ins.len();

    while i < ins_length {
        let def = {
            let result = lookup(ins[i]);

            if let Ok(it) = result {
                it
            } else if let Err(msg) = result {
                s.push_str(&format!("Error: {}", msg.red()));
                continue;
            } else {
                continue;
            }
        };
        
        let result = read_operands(&def, &ins[(i + 1)..].to_vec());

        let operands = result.0;
        let read = result.1;

        s.push_str(&format!("{:04} {}\n", i, fmt_instruction(&def, &operands)));

        i += 1 + read
    }

    s
}

pub fn fmt_instruction(def: &Definition, operands: &Vec<i32>) -> String {
    let operand_count = def.operand_widths.len();

    if operands.len() != operand_count {
        return format!("ERROR: operand len {} does not match defined {}\n", operands.len(), operand_count)
    }

    match operand_count {
        0 => def.name.clone(),
        1 => format!("{} {}", def.name, operands[0]),
        _ => format!("ERROR: unhandled operandCount for {}\n", def.name),
    }
}

pub fn read_operands(def: &Definition, ins: &Instructions) -> (Vec<i32>, usize) {
    let mut operands = Vec::with_capacity(def.operand_widths.len());
    let mut offset = 0;

    for &width in &def.operand_widths {
        match width {
            2 => {
                let bytes = &ins[offset..offset + width as usize];
                operands.push(i32::from(BigEndian::read_u16(&bytes)));
            }
            _ => {}
        }
        offset += width as usize;
    }

    (operands, offset)
}

pub fn read_uint16(ins: &[u8]) -> u16 {
    BigEndian::read_u16(ins)
}

