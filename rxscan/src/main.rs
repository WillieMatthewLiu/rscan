mod logging;
mod global;

use clap::{Parser, Subcommand};
use tracing::*;
use anyhow::Result;

use crate::{global::{SCAN_CONTEXT, ScanContext}, logging::init_logging};

#[derive(Parser, Debug)]
#[command(
    author = "尼古拉斯.拖延蟹",
    version = "1.0",
    about = "这是一个Rust编写的插件化单机扫描工具,开始参考了FScan等知名开源项目",
    long_about = "这是一个Rust编写的插件化单机扫描工具,开始参考了FScan等知名开源项目"
)]
pub struct Cli{
    /// 详细模式（For a flag that defaults to false and becomes true when present）
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    verbose: Option<bool>,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands{
    // 端口扫描
    port {
        // 目标主机或者网段
        target: String,
         
        // 端口范围
        #[arg(short, long, default_value = "1-65535")]
        ports: String,
    },
    // web扫描
    web {

    },
    // 系统扫描
    sys {

    },

}
/// 初始扫描的全局变量
fn build_global_scan_context(cli: &Cli) -> ScanContext{
    let mut scan_context = ScanContext::default();

    if let Some(verbose) = cli.verbose{
        scan_context.verbose = verbose;
    }
    
    scan_context
}

async fn _main()-> Result<()> {
    let scan_cli = Cli::parse();
    // 初始化系统日志组件
    init_logging().await?;

    // 初始化全局配置元素
    let scan_context = build_global_scan_context(&scan_cli);

    SCAN_CONTEXT.set(scan_context).expect("初始化扫描全局变量失败");

    // 匹配命令执行
    match &scan_cli.command {
        _ => {
            // 确保有返回值
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() {
    // 工具主入口
    if let Err(error) = _main().await {
        error!(?error, "系统主函数初始化异常");
        std::process::exit(1);
    }
}