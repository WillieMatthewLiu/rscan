use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author = "尼古拉斯.拖延蟹 <2004200908@163.com>",
    version = "1.0",
    about = "这是我用rust编写的第一个安全工具",
    long_about = "这是我用rust编写的第一个安全工具"
)]
pub struct Cli{
    /// 详细模式
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands{

}