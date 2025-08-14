use std::{cell::RefCell, rc::Rc};

use crate::{byte_code_vm::code::code::{instruction_to_str_with_indent, Instructions}, object::ant_compiled_function::CompiledFunction};

#[derive(Clone, Debug)]
pub struct Frame {
    pub func: Rc<RefCell<CompiledFunction>>,
    pub ip: isize,
    pub base_pointer: usize,
}

impl Frame {
    pub fn new(func: Rc<RefCell<CompiledFunction>>, base_pointer: usize) -> Self {
        Self {
            func,
            ip: -1,
            base_pointer
        }
    }

    pub fn instructions(&self) -> Rc<RefCell<Instructions>> {
        self.func.borrow().instructions.clone()
    }
}

pub fn fmt_compiled_function(func: Rc<RefCell<CompiledFunction>>, indent: &str) -> String {
    let mut s = String::new();

    let borrow_func = func.borrow();

    s.push_str("CompiledFunction: \n");
    s.push_str(&format!("{indent}Instructions:\n"));
    s.push_str(&format!(
        "{}\n", 
        instruction_to_str_with_indent(
            &borrow_func
                .instructions
                .borrow()
                .clone(),
            &indent.repeat(2)
        )
    ));

    s
}

pub fn fmt_frames(frames: &Vec<Frame>) -> String {
    let mut s = String::new();

    for (index, f) in frames.iter().enumerate() {
        s.push_str(&format!("Frame{index}: \n"));
        s.push_str(&format!("    {}\n", fmt_compiled_function(f.func.clone(), "\t")));
        s.push_str(&format!("    InstructionsPos: {}\n", f.ip));
    }

    s
}