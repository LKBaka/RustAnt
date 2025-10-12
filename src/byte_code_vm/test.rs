#[cfg(test)]
mod tests {
    const TEST_COUNT: u16 = 1000;

    use std::time::{Duration, Instant};

    use bigdecimal::BigDecimal;
    use colored::Colorize;

    use crate::byte_code_vm::{compiler::utils::compile_it, constants::UNINIT_OBJECT, vm::vm::{Vm, GLOBALS_SIZE}};
    use crate::rc_ref_cell;

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

            let mut globals = vec![rc_ref_cell!(UNINIT_OBJECT.clone()); GLOBALS_SIZE as usize];
            let mut vm = Vm::new(bytecode.clone(), &mut globals);
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