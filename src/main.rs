mod runner;
mod arg_structure;
mod token;
mod lexer;
mod constants;
mod utils;
mod parser;
mod ast;
mod environment;
mod object;
mod map;
mod char_string;
mod module_system;
mod byte_code_vm;
// mod lg_ir_gen;

extern crate lazy_static;

use clap::Parser;
use arg_structure::arg_structure::Args;

use crate::runner::file_runner::FileRunner;
use crate::runner::repl_runner::REPLRunner;

fn main() {
    let args = Args::parse();

    // 判断是否需要进入REPL
    if args.file.is_none() { // 没有提供文件路径
        // 进入REPL
        let repl = REPLRunner::new(args);
        repl.run();
    } else {
        let file_runner = FileRunner::new(args.file.clone().unwrap(), args.clone());
        file_runner.run();
    }
}
