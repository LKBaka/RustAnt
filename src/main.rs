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
mod gc;

extern crate lazy_static;

use clap::Parser;
use arg_structure::arg_structure::Args;
use gc::gc::set_max_recursion_depth;

use crate::runner::file_runner::FileRunner;
use crate::runner::repl_runner::REPLRunner;

fn main() {
    let args = Args::parse();

    set_max_recursion_depth(args.max_recursion_depth);

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
