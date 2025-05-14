use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "RustAnt",
    version = "1.0.0",
    about = "AntScript on Rust",
    long_about = None
)]
pub struct Args {
    /// 输入文件路径（可选）
    #[arg(short, long)]
    pub(crate) file: Option<String>,

    /// GC可容忍最大递归次数（可选）
    #[arg(short, long, default_value_t = 700)]
    pub(crate) max_recursion_depth: usize,

    /// 启用详细模式
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// 处理次数
    #[arg(short, long, default_value_t = 1)]
    count: u32,
}
