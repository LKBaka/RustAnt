use std::io;
use std::io::Write;
use crate::environment::environment::Environment;
use crate::object::ant_string::AntString;
use crate::object::object::IAntObject;

pub fn builtin_print(arg_env: &mut Environment) -> Option<Box<dyn IAntObject>> {
    let value = arg_env.get("value");
    match value {
        None => {
            panic!("what the fuck? arg_env: {}", arg_env.to_string())
        }
        Some(it) => {
            println!("{}", it.inspect());
        }
    }

    None
}

pub fn builtin_input(arg_env: &mut Environment) -> Option<Box<dyn IAntObject>> {
    let value = arg_env.get("prompt");
    match value {
        None => {
            let mut input = "".to_string();

            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            Some(AntString::new_with_native_value(Box::new(input)))
        }
        Some(it) => {
            print!("{}", it.inspect());

            let mut input = "".to_string();

            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            Some(AntString::new_with_native_value(Box::new(input)))
        }
    }
}