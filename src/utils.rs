use std::{
    io,
    process::{Command, ExitStatus, Stdio},
};

#[cfg(test)]
use std::fmt::Debug;

#[cfg(test)]
pub fn assert_eq<T, F>(left: T, right: T, on_failure: F)
where
    T: PartialEq + Debug, // 允许比较和格式化输出
    F: FnOnce(),          // 接受无参闭包/函数指针
{
    if left != right {
        on_failure(); // 断言失败时调用回调函数
        panic!(
            "assertion failed: left != right\n  left: `{:?}`,\n right: `{:?}`",
            left, right
        );
    }
}

#[macro_export]
macro_rules! struct_type_id {
    ($t:ty) => {{ std::any::TypeId::of::<$t>() }};
}

pub fn run_command(command: &str) -> io::Result<ExitStatus> {
    // 根据不同操作系统选择 shell
    #[cfg(windows)]
    let (shell, flag) = ("cmd.exe", "/C");

    #[cfg(not(windows))]
    let (shell, flag) = ("/bin/sh", "-c");
    
    // 构建并执行命令
    let status = Command::new(shell)
        .arg(flag)
        .arg(command)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| {
            eprintln!("run '{}' failed: {}", command, e);
            e
        });

    status
}

#[macro_export]
macro_rules! hash_map {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

#[macro_export]
macro_rules! rc_ref_cell {
    ($value:expr) => {
        std::rc::Rc::new(std::cell::RefCell::new($value))
    };
}

#[macro_export]
macro_rules! ant_assert {
    ($expr:expr) => {
        if !($expr) {
            Err(format!("assertion failed: {}", stringify!($expr)))?
        }
    };
}

#[test]
fn test_ant_assert() -> Result<(), String> {
    ant_assert!(1 == 1);

    Ok(())
}