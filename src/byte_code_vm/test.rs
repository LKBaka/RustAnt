#[cfg(feature = "byte_code_rust_ant")]
pub fn test_byte_code_rust_ant_main() {
    use std::io;

    use std::io::Write;

    let mut code = String::new();

    let file = "repl".to_string();

    loop {
        // get user input (repl)
        print!(">>> ");

        let flush_result = io::stdout().flush(); // 刷新缓冲区（重要！）
        match flush_result {
            Ok(_) => {},
            Err(e) => {eprintln!("{}", e.to_string()); continue;}
        }

        let read_line_result = io::stdin().read_line(&mut code);
        match read_line_result {
            Err(e) => {eprintln!("{}", e.to_string()); continue;},
            Ok(_) => {}
        }


        #[cfg(feature = "get_code_run_seconds")]
        use std::time::Instant;

        use crate::byte_code_vm::run::run;

        #[cfg(feature = "get_code_run_seconds")]
        let start = Instant::now();

        let result = run(code.clone(), file.clone());
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
            "{}", format!(
                "(Use Compiler And VM (ByteCode)) Code run time: {} seconds, {} milliseconds, {} nanoseconds", 
                start.elapsed().as_secs_f64(), start.elapsed().as_millis(), start.elapsed().as_nanos()
            )
        );

        code = String::new();
    }
}