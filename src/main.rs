mod arg_structure;
mod ast;
mod byte_code_vm;
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

extern crate lazy_static;

use std::path::PathBuf;

use arg_structure::arg_structure::Args;
use clap::Parser;

use crate::constants::MODULE_PATHS;
use crate::obj_enum::object::Object;
use crate::object::ant_string::AntString;
use crate::runner::file_runner::FileRunner;
use crate::runner::repl_runner::REPLRunner;

fn main() {
    if let Some(it) = global_env::get_global_env("ANTMAN_PATH") {
        let o = Object::AntString(AntString::new(
            PathBuf::from(it)
                .join("modules")
                .to_str()
                .unwrap()
                .to_string()
        ));

        MODULE_PATHS.lock().unwrap().items.push(o);
    }

    let args = Args::parse();

    // 判断是否需要进入REPL
    if args.file.is_none() {
        // 没有提供文件路径
        // 进入REPL
        let repl = REPLRunner::new();
        repl.run();
    } else {
        let o = Object::AntString(AntString::new(
            PathBuf::from(&args.file.clone().unwrap())
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        ));

        MODULE_PATHS.lock().unwrap().items.push(o);

        let file_runner = FileRunner::new(args.file.clone().unwrap());
        file_runner.run();
    }
}
