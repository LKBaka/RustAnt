use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use bigdecimal::{BigDecimal, FromPrimitive};

use crate::environment::environment::Environment;
use crate::extract_arg;
use crate::object::ant_double::AntDouble;
use crate::object::ant_string::AntString;
use crate::object::object::Object;
use crate::utils::run_command;

pub fn builtin_print(arg_env: &mut Environment) -> Option<Object> {
    let value = arg_env.get("value").expect(
        &format!("cannot find \"value\". arg_env: {}", arg_env.to_string())
    );

    let end = arg_env.get("end").expect(
        &format!("cannot find \"end\". arg_env: {}", arg_env.to_string())
    );

    print!("{}{}", value.inspect(), end.inspect());

    None
}

pub fn builtin_input(arg_env: &mut Environment) -> Option<Object> {
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

pub fn builtin_clear(arg_env: &mut Environment) -> Option<Object> {
    print!("\x1b[2J");
    print!("\x1b[H");

    None
}

pub fn builtin_shell(arg_env: &mut Environment) -> Option<Object> {
    let command = extract_arg!(arg_env, "command" => AntString)?;

    let result = run_command(command.value.as_str());

    None
}

pub fn builtin_now(_arg_env: &mut Environment) -> Option<Object> {
    Some(
        AntDouble::new_with_native_value(Box::new(BigDecimal::from_f64(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| {
                    duration
                        .as_secs() as f64
                        + duration
                            .subsec_nanos() as f64
                            / 1_000_000_000.0
                })
                .unwrap_or(-1.0)
        ).unwrap()))
    )
}