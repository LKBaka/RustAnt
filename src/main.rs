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
mod evaluator;
mod char_string;
mod function_caller;

extern crate lazy_static;

use clap::Parser;
use arg_structure::arg_structure::Args;
use crate::runner::file_runner::FileRunner;
use crate::runner::repl_runner::REPLRunner;

fn main() {
    let args = Args::parse();

    // 判断是否需要进入REPL
    if args.file == None { // 没有提供文件路径
        // 进入REPL
        let repl = REPLRunner::new();
        repl.run();
    } else {
        let file_runner = FileRunner::new(args.file.unwrap());
        file_runner.run();
    }
}
