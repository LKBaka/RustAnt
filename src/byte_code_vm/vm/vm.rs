use std::{cell::RefCell, rc::Rc};

use crate::{
    byte_code_vm::{
        code::code::{
            read_uint16, read_uint8, OP_ADD, OP_ARRAY, OP_BANG, OP_CALL, OP_CONSTANTS, OP_FALSE, OP_GET_GLOBAL, OP_GET_LOCAL, OP_INDEX, OP_JUMP, OP_JUMP_NOT_TRUTHY, OP_MINUS, OP_NOP, OP_NOTEQ, OP_POP, OP_RETURN_VALUE, OP_SET_GLOBAL, OP_SET_LOCAL, OP_TEST_PRINT, OP_TRUE
        },
        compiler::compiler::ByteCode,
        constants::{FALSE, TRUE, UNINIT_OBJ},
        vm::{
            eval_functions::{
                eval_array_literal_utils::build_array,
                eval_index_expression::eval_index_expression,
                eval_infix_operator::eval_infix_operator,
                eval_prefix_operator::eval_prefix_operator,
            },
            frame::Frame,
        },
    },
    object::{
        ant_compiled_function::CompiledFunction,
        object::{Object, UNINIT}, utils::is_truthy,
    },
    rc_ref_cell,
};

pub const STACK_SIZE: u16 = 2048;
pub const GLOBALS_SIZE: u16 = 65535;

pub struct Vm {
    constants: Vec<Object>,
    stack: Vec<Object>,
    globals: Rc<RefCell<Vec<Rc<RefCell<Object>>>>>,
    frames: Vec<Rc<RefCell<Frame>>>,
    frame_index: usize,
    sp: usize // stack next pos
}

impl Vm {
    pub fn new(bytecode: ByteCode) -> Self {
        let uninit: Vec<Rc<RefCell<Object>>> = vec![rc_ref_cell!(Box::new(UNINIT_OBJ.clone()))];

        let main_func = CompiledFunction {
            instructions: rc_ref_cell!(bytecode.instructions),
            local_count: 0,
            param_count: 0,
        };

        let main_frame = Frame::new(rc_ref_cell!(main_func), 0);

        Vm {
            constants: bytecode.constants,
            stack: vec![],
            globals: rc_ref_cell!(vec![uninit[0].clone(); GLOBALS_SIZE as usize]),
            frames: vec![rc_ref_cell!(main_frame)],
            frame_index: 1,
            sp: 0
        }
    }

    pub fn with_globals(
        bytecode: ByteCode,
        globals: Rc<RefCell<Vec<Rc<RefCell<Object>>>>>,
    ) -> Self {
        let main_func = CompiledFunction {
            instructions: rc_ref_cell!(bytecode.instructions),
            local_count: 0,
            param_count: 0,
        };

        let main_frame = Frame::new(rc_ref_cell!(main_func), 0);

        Vm {
            constants: bytecode.constants,
            stack: vec![],
            globals,
            frames: vec![rc_ref_cell!(main_frame)],
            frame_index: 1,
            sp: 0
        }
    }

    pub fn current_frame(&self) -> Rc<RefCell<Frame>> {
        self.frames[self.frame_index - 1].clone()
    }

    pub fn push_frame(&mut self, frame: Rc<RefCell<Frame>>) {
        if self.frame_index >= self.frames.len() {
            self.frames.push(frame.clone());
        } else {
            self.frames[self.frame_index] = frame.clone();
        }

        self.frame_index += 1;
    }

    pub fn pop_frame(&mut self) -> Rc<RefCell<Frame>> {
        self.frame_index -= 1;
        self.frames[self.frame_index].clone()
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut ip;

        let mut instructions;

        let mut op;

        while self.current_frame().borrow().ip
            < self.current_frame().borrow().instructions().borrow().len() as isize - 1
        {
            self.current_frame().borrow_mut().ip += 1;

            ip = self.current_frame().borrow().ip as usize;

            instructions = self.current_frame().borrow().instructions();

            op = instructions.borrow()[ip];

            match op {
                OP_CONSTANTS => {
                    let const_index = read_uint16(&instructions.borrow()[(ip + 1)..]);
                    self.current_frame().borrow_mut().ip += 2;

                    let result = self.push(self.constants[const_index as usize].clone());

                    if result.is_err() {
                        return result;
                    }
                }

                OP_POP => {
                    self.pop();
                }

                OP_ADD..=OP_NOTEQ => {
                    let right = self.pop();
                    let left = self.pop();

                    if left.is_none() || right.is_none() {
                        return Err(format!(
                            "expected two objects of opcode {}. got {} objects",
                            op,
                            left.is_some() as u8 + right.is_some() as u8
                        ));
                    }

                    let left_obj = left.unwrap();
                    let right_obj = right.unwrap();

                    let eval_operator_result = eval_infix_operator(op, left_obj, right_obj);

                    if let Err(err) = eval_operator_result {
                        return Err(format!("error evaluating infix operator {}: {}", op, err));
                    }

                    let push_result = self.push(eval_operator_result?);

                    if push_result.is_err() {
                        return push_result;
                    }
                }

                OP_TRUE..=OP_FALSE => {
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

                OP_MINUS..=OP_BANG => {
                    let right = match self.pop() {
                        Some(obj) => obj,
                        None => return Err(format!("expected an object for opcode {}", op)),
                    };

                    let eval_operator_result = eval_prefix_operator(op, right);

                    if let Err(err) = eval_operator_result {
                        return Err(format!("error evaluating prefix operator {}: {}", op, err));
                    }

                    let push_result = self.push(eval_operator_result.unwrap());

                    if push_result.is_err() {
                        return push_result;
                    }
                }

                OP_SET_GLOBAL => {
                    let global_index = read_uint16(&instructions.borrow()[(ip + 1)..]);

                    self.current_frame().borrow_mut().ip += 2;

                    if let Some(obj) = self.pop() {
                        self.globals.borrow_mut()[global_index as usize] = rc_ref_cell!(obj);
                    }
                }

                OP_GET_GLOBAL => {
                    let global_index = read_uint16(&instructions.borrow()[(ip + 1)..]);
                    self.current_frame().borrow_mut().ip += 2;

                    let obj_clone = {
                        let obj: &Rc<RefCell<Object>> =
                            &self.globals.borrow_mut()[global_index as usize];

                        if obj.borrow().get_type() != UNINIT {
                            Some(obj.borrow().clone())
                        } else {
                            None
                        }
                    };

                    if let Some(obj) = obj_clone {
                        if let Err(msg) = self.push(obj) {
                            return Err(format!("error push global variable: {}", msg));
                        }
                    }
                }

                OP_ARRAY => {
                    let array_len = read_uint16(&instructions.borrow()[(ip + 1)..]);
                    self.current_frame().borrow_mut().ip += 2;

                    let array_obj = build_array(
                        &self.stack,
                        self.sp - array_len as usize,
                        self.sp,
                    );

                    self.sp -= array_len as usize;

                    let push_result = self.push(Box::new(array_obj));
                    if let Err(msg) = push_result {
                        return Err(format!("error push array object: {msg}"));
                    }
                }

                OP_INDEX => {
                    let index = self.pop();
                    let left = self.pop();

                    if left.is_none() || index.is_none() {
                        return Err(format!(
                            "expected two objects (index, left) {}. got {} objects",
                            op,
                            left.is_some() as u8 + index.is_some() as u8
                        ));
                    }

                    let left_obj = left.unwrap();
                    let index_obj = index.unwrap();

                    match eval_index_expression(left_obj, index_obj) {
                        Ok(obj) => {
                            if let Err(msg) = self.push(obj) {
                                return Err(format!("error push object: {msg}"));
                            }
                        }

                        Err(msg) => {
                            return Err(format!("error evaluating index expression: {msg}"));
                        }
                    }
                }

                OP_CALL => {
                    let arg_count = read_uint8(&instructions.borrow()[(ip + 1)..]);
                    self.current_frame().borrow_mut().ip += 1;
                    
                    if let Err(msg) = self.call_function(arg_count as usize) {
                        return Err(format!("error calling function: {msg}"))
                    }
                }

                OP_RETURN_VALUE => {
                    let return_value = self.pop();

                    let frame = self.pop_frame(); // 弹出当前帧
                    self.sp = frame.borrow().base_pointer - 1;

                    if let Some(value) = return_value {
                        if let Err(msg) = self.push(value) {
                            return Err(format!("error push return value: {msg}"));
                        }
                    }
                }

                OP_SET_LOCAL => {
                    let local_index = read_uint16(&instructions.borrow()[(ip + 1)..]);

                    self.current_frame().borrow_mut().ip += 2;

                    let frame = self.current_frame();

                    let index = frame.borrow().base_pointer + local_index as usize;

                    self.stack[index] = self.pop().expect("expected an object to set local")
                }

                OP_GET_LOCAL => {
                    let local_index = read_uint16(&instructions.borrow()[(ip + 1)..]);

                    self.current_frame().borrow_mut().ip += 2;

                    let frame = self.current_frame();

                    let index = frame.borrow().base_pointer + local_index as usize;

                    if let Err(msg) = self.push(self.stack[index].clone()) {
                        return Err(format!("error get local variable: {msg}"));
                    }
                }

                OP_JUMP => {
                    let jump_to = read_uint16(&instructions.borrow()[(ip + 1)..]);
                    self.current_frame().borrow_mut().ip += 2;

                    self.current_frame().borrow_mut().ip = (jump_to as isize) - 1
                }

                OP_JUMP_NOT_TRUTHY => {
                    let jump_to = read_uint16(&instructions.borrow()[(ip + 1)..]);
                    self.current_frame().borrow_mut().ip += 2;
                    
                    let condition = if let Some(cond) = self.pop() {
                        cond
                    } else {
                        return Err(String::from("expected an condition"))
                    };

                    if !is_truthy(&condition) {
                        self.current_frame().borrow_mut().ip = (jump_to as isize) - 1;
                    }
                }

                OP_TEST_PRINT => {
                    let obj = if let Some(obj) = self.pop() {
                        obj
                    } else {
                        return Err(String::from("expected an object to print"))
                    };

                    println!("{}", obj.inspect());
                }

                OP_NOP => {}

                _ => return Err(format!("unknown opcode: {}", op)),
            }
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
        if self.sp == 0 {
            None
        } else {
            Some(self.stack[self.sp].clone())
        }
    }

    #[inline]
    pub fn push(&mut self, obj: Object) -> Result<(), String> {
        if self.sp >= STACK_SIZE as usize {
            return Err("Stack overflow".to_string());
        }

        if self.sp >= self.stack.len() {
            self.stack.resize(
                self.sp + 1,
                Box::new(UNINIT_OBJ.clone()),
            );
        }
        self.stack[self.sp] = obj;

        self.sp += 1;

        Ok(())
    }

    #[inline]
    pub fn pop(&mut self) -> Option<Object> {
        if self.sp == 0 {
            None
        } else {
            self.sp -= 1;

            // 地雷式 stack, 不是合法的 sp 范围直接 panic
            Some(self.stack[self.sp].clone())
        }
    }

    pub fn frames(&self) -> Vec<Frame> {
        self.frames.iter().map(|f| f.borrow().clone()).collect()
    }

    pub fn call_function(&mut self, arg_count: usize) -> Result<(), String> {
        let calling_obj = if let Some(it) = self.stack[self.sp - 1 - arg_count]
            .as_any()
            .downcast_ref::<CompiledFunction>()
        {
            it.clone()
        } else {
            return Err(format!("calling non-function"));
        };

        if arg_count != calling_obj.param_count {
            return Err(format!("expected {arg_count} args, got {} args", calling_obj.param_count))
        }

        let frame = Frame::new(rc_ref_cell!(calling_obj.clone()), self.sp - arg_count);
        self.push_frame(rc_ref_cell!(frame.clone()));

        self.sp = frame.base_pointer + calling_obj.local_count;

        Ok(())
    }
}
