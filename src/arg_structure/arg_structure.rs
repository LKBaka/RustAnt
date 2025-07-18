use clap::Parser;

#[derive(Parser, Debug, Clone)]
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

    /// 是否在执行前打印AST（可选）
    #[arg(short, long, default_value_t = false)]
    pub(crate) print_ast: bool,
}
