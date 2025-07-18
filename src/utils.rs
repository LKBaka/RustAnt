use std::{any::{Any, TypeId}, io, process::{Command, ExitStatus, Stdio}};


#[cfg(test)]
use std::fmt::Debug;

#[cfg(test)]
pub fn assert_eq<T, F>(left: T, right: T, on_failure: F)
where
    T: PartialEq + Debug,        // 允许比较和格式化输出
    F: FnOnce(),                 // 接受无参闭包/函数指针
{
    if left != right {
        on_failure();            // 断言失败时调用回调函数
        panic!("assertion failed: left != right\n  left: `{:?}`,\n right: `{:?}`", left, right);
    }
}

pub fn type_of<T: Any>(_: &T) -> TypeId {
    TypeId::of::<T>()
}

pub fn run_command(command: &str) -> io::Result<ExitStatus> {
    // 根据不同操作系统选择 shell
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd.exe", "/C")
    } else {
        ("/bin/sh", "-c")
    };
    
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