use std::rc::Rc;

use crate::{
    byte_code_vm::code::code::{Instructions, instruction_to_str_with_indent},
    object::{ant_closure::Closure, ant_compiled_function::CompiledFunction},
};

#[derive(Clone, Debug, Hash)]
pub struct Frame {
    pub closure: Closure,
    pub ip: isize,
    pub base_pointer: usize,
}

impl Frame {
    pub fn new(closure: Closure, base_pointer: usize) -> Self {
        Self {
            closure,
            ip: -1,
            base_pointer,
        }
    }

    #[inline(always)]
    pub fn instructions(&self) -> Rc<Instructions> {
        self.closure.func.instructions.clone()
    }
}

pub fn fmt_compiled_function(func: CompiledFunction, indent: &str) -> String {
    let mut s = String::new();

    s.push_str("CompiledFunction: \n");
    s.push_str(&format!("{indent}Instructions:\n"));
    s.push_str(&format!(
        "{}\n",
        instruction_to_str_with_indent(&func.instructions, &indent.repeat(2))
    ));
    s.push_str(&format!("{indent}RuntimeInfo:\n"));
    s.push_str(&format!("{}{:#?}\n", &indent.repeat(2), func.scope_info));

    s
}

pub fn fmt_closure(closure: Closure, indent: &str) -> String {
    let mut s = String::new();

    s.push_str("Closure: \n");
    s.push_str(&format!(
        "{indent}{}\n",
        fmt_compiled_function(closure.func.clone(), indent)
    ));
    s.push_str(&format!("{indent}{:#?}\n", closure.free.clone()));

    s
}

pub fn fmt_frames(frames: &Vec<Frame>) -> String {
    let mut s = String::new();

    for (index, f) in frames.iter().enumerate() {
        s.push_str(&format!("Frame{index}: \n"));
        s.push_str(&format!("    {}\n", fmt_closure(f.closure.clone(), "\t")));
        s.push_str(&format!("    InstructionsPos: {}\n", f.ip));
        s.push_str(&format!("    BasePointer: {}\n", f.base_pointer));
    }

    s
}
