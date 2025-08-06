use crate::{byte_code_vm::{code::code::{read_uint16, Instructions, OP_ADD, OP_BANG, OP_CONSTANTS, OP_FALSE, OP_MINUS, OP_NOTEQ, OP_POP, OP_TRUE}, compiler::compiler::ByteCode, constants::{FALSE, TRUE}, vm::{eval_infix_operator::eval_infix_operator, eval_prefix_operator::eval_prefix_operator}}, object::object::Object};


pub const STACK_SIZE: i32 = 2048;

pub struct Vm {
    constants: Vec<Object>,
    instructions: Instructions,
    stack: Vec<Object>,
    sp: usize, // stack pointer, always points to the next empty slot in the stack
}

impl Vm {
    pub fn new(bytecode: ByteCode) -> Self {
        Vm {
            constants: bytecode.constants,
            instructions: bytecode.instructions,
            stack: Vec::with_capacity((STACK_SIZE / 16) as usize), 
            sp: 0, // stack pointer starts at 0
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut ip = 0;
        while ip < self.instructions.len() {
            let op = self.instructions[ip];

            match op {
                OP_CONSTANTS => {
                    let const_index = read_uint16(&self.instructions[(ip+1)..]) ;
                    ip += 2;

                    let result = self.push(self.constants[const_index as usize].clone());

                    if result.is_err() {
                        return result;
                    }
                }

                OP_POP => {
                    self.pop();
                }

                OP_ADD ..= OP_NOTEQ => {
                    let right = self.pop();
                    let left = self.pop();

                    if left.is_none() || right.is_none() {
                        return Err(format!(
                            "Expected two objects of opcode {}. Got {} objects", 
                            op, left.is_some() as i32 + right.is_some() as i32
                        ));
                    }

                    let left_obj = left.unwrap();
                    let right_obj = right.unwrap();

                    let eval_operator_result = eval_infix_operator(op, left_obj, right_obj);

                    if let Err(err) = eval_operator_result {
                        return Err(format!("Error evaluating infix operator {}: {}", op, err));
                    }

                    let push_result = self.push(eval_operator_result.unwrap());

                    if push_result.is_err() {
                        return push_result;
                    }
                }

                OP_TRUE ..= OP_FALSE => {
                    let obj = if op == OP_TRUE {
                        TRUE.clone()
                    } else {
                        FALSE.clone()
                    };

                    let push_result = self.push(Box::new(obj));

                    if push_result.is_err() {
                        return push_result;
                    }
                }

                OP_MINUS ..= OP_BANG => {
                    let right = match self.pop() {
                        Some(obj) => obj,
                        None => return Err(format!("Expected an object for opcode {}", op)),
                    };

                    let eval_operator_result = eval_prefix_operator(op, right);

                    if let Err(err) = eval_operator_result {
                        return Err(format!("Error evaluating prefix operator {}: {}", op, err));
                    }

                    let push_result = self.push(eval_operator_result.unwrap());

                    if push_result.is_err() {
                        return push_result;
                    }
                }

                _ => return Err(format!("Unknown opcode: {}", op)),
            }

            ip += 1;
        }
        
        Ok(())
    }

    pub fn stack_top(&self) -> Option<Object> {
        if self.sp == 0 {
            None
        } else {
            Some(self.stack[self.sp - 1].clone())
        }
    }

    pub fn last_popped_stack_elem(&self) -> Option<Object> {
        if self.sp >= self.stack.len() {
            None
        } else {
            Some(self.stack[self.sp].clone())
        }
    }

    pub fn push(&mut self, obj: Object) -> Result<(), String> {
        if self.sp >= STACK_SIZE as usize {
            return Err("Stack overflow".to_string());
        }

        if self.sp >= self.stack.len() {
            self.stack.push(obj);
        } else {
            self.stack[self.sp] = obj;
        }

        self.sp += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Option<Object> {
        if self.sp == 0 {
            None
        } else {
            self.sp -= 1;
            Some(self.stack[self.sp].clone())
        }
    }
}