#[cfg(feature = "byte_code_rust_ant")]
pub fn test_byte_code_rust_ant_main() {
    use std::io;

    use std::io::Write;

    let mut code = String::new();

    loop {
        // get user input (repl)
        io::stdin()
            .read_line(&mut code)
            .expect("Failed to read from stdin");

        
        io::stdout()
            .flush()
            .expect("Failed to flush stdout");

        println!("{}", code);
    }
}