use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "RustAnt",
    version = "0.1.0",
    about = "AntScript on Rust",
    long_about = None
)]
pub struct Args {
    /// 输入文件路径（可选）
    #[arg(short, long)]
    pub(crate) file: Option<String>,
}
