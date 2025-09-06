mod arg_structure;
mod ast;
mod byte_code_vm;
mod char_string;
mod constants;
mod lexer;
mod object;
mod parser;
mod runner;
mod token;
mod utils;
mod builtin;
mod obj_enum;
mod module_importer;
// mod lg_ir_gen;

extern crate lazy_static;

use arg_structure::arg_structure::Args;
use clap::Parser;

use crate::runner::file_runner::FileRunner;
use crate::runner::repl_runner::REPLRunner;

fn main() {
    let args = Args::parse();

    // 判断是否需要进入REPL
    if args.file.is_none() {
        // 没有提供文件路径
        // 进入REPL
        let repl = REPLRunner::new();
        repl.run();
    } else {
        let file_runner = FileRunner::new(args.file.clone().unwrap());
        file_runner.run();
    }
}
