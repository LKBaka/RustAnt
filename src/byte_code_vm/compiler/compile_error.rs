use std::fmt::Display;

use crate::byte_code_vm::compiler::compiler::CompileError;

#[derive(Debug)]
pub struct CompileErrorBox {
    pub compile_error: CompileError,
    pub traceback_string: String
}

impl CompileErrorBox {
    pub fn from_traceback_string(
        err: CompileError,
        traceback_string: String
    ) -> Self {
        Self {
            compile_error: err,
            traceback_string
        }
    }
}

impl Display for CompileErrorBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.compile_error.token {
            Some(token) => write!(
                f,
                "{}\n{}\n(at line: {}, at column: {})",
                self.traceback_string,
                self.compile_error.message, 
                token.line, token.column
            ),

            None => write!(f, "{}\n{}", self.traceback_string, self.compile_error.message),
        }
    }
}