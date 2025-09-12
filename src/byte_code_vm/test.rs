#[cfg(test)]
pub fn test_byte_code_rust_ant_main() {
    use std::cell::RefCell;
    use std::io;

    use std::io::Write;
    use std::rc::Rc;

    use crate::byte_code_vm::compiler::symbol_table::symbol_table::SymbolTable;
    use crate::byte_code_vm::constants::UNINIT_OBJ;
    use crate::byte_code_vm::vm::vm::GLOBALS_SIZE;
    use crate::rc_ref_cell;

    use crate::obj_enum::object::Object;
    use crate::object::object::IAntObject;

    let mut code;

    let file = "repl".to_string();

    let uninit: Vec<Rc<RefCell<Object>>> = vec![rc_ref_cell!(Object::AntUninit(UNINIT_OBJ.clone()))];

    let symbol_table = rc_ref_cell!(SymbolTable::new());
    let constants = rc_ref_cell!(vec![]);
    let globals = rc_ref_cell!(vec![uninit[0].clone(); GLOBALS_SIZE as usize]);

    loop {
        code = String::new();

        // get user input (repl)
        print!(">>> ");

        let flush_result = io::stdout().flush(); // 刷新缓冲区（重要！）
        match flush_result {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e.to_string());
                continue;
            }
        }

        let read_line_result = io::stdin().read_line(&mut code);
        match read_line_result {
            Err(e) => {
                eprintln!("{}", e.to_string());
                continue;
            }
            Ok(_) => {}
        }

        #[cfg(feature = "get_code_run_seconds")]
        use std::time::Instant;

        use crate::byte_code_vm::run::run;

        #[cfg(feature = "get_code_run_seconds")]
        let start = Instant::now();

        let result = run(
            code.clone(),
            file.clone(),
            symbol_table.clone(),
            constants.clone(),
            globals.clone(),
        );

        if let Err(err_enum) = result {
            use crate::byte_code_vm::run::RunError;

            if let RunError::CompileError(msg) = err_enum {
                use colored::Colorize;

                eprintln!("{}", msg.red());
            } else if let RunError::RuntimeError(err) = err_enum {
                use colored::Colorize;

                eprintln!("{}", err.inspect().red());
            }
        } else if let Ok(Some(result)) = result {
            println!("{}", result.inspect());
        }

        #[cfg(feature = "get_code_run_seconds")]
        println!(
            "{}",
            format!(
                "(Use Compiler And VM (ByteCode)) Code run time: {} seconds, {} milliseconds, {} nanoseconds",
                start.elapsed().as_secs_f64(),
                start.elapsed().as_millis(),
                start.elapsed().as_nanos()
            )
        );
    }
}

#[cfg(test)]
mod tests {
    const TEST_COUNT: u16 = 1000;

    use std::time::{Duration, Instant};

    use bigdecimal::BigDecimal;
    use colored::Colorize;

    use crate::byte_code_vm::compiler::utils::compile_it;
    use crate::byte_code_vm::vm::vm::Vm;

    fn test_speed() {
        let bytecode = compile_it(
            r#"
            let sum = 0; let i = 1;
            while i < 10001 { 
                i = i + 1
                sum = i + sum
            }
            "#.into(), 
            "__test_speed__".into()
        ).unwrap();

        let mut total: Duration = Default::default();

        for _ in 0..TEST_COUNT {
            let start = Instant::now();

            let mut vm = Vm::new(bytecode.clone());
            vm.run().expect("an error of vm");

            total += start.elapsed()
        }

        println!("{}", format!(
            r#"
total seconds: {}, total millseconds: {}, total nanoseconds: {}
average seconds: {}, average millseconds: {}, average nanoseconds: {}
            "#,
            total.as_secs_f64(),
            total.as_millis(),
            total.as_nanos(),
            total.as_secs_f64() / TEST_COUNT as f64,
            BigDecimal::from(total.as_millis()) / TEST_COUNT as f64,
            BigDecimal::from(total.as_nanos()) / TEST_COUNT as f64
        ).green())
    }
}