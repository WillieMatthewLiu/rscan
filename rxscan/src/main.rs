//! 作者: 拖延蟹@B1tf0rce
//! 创建日期: 2025-11-06
//! 最后修改: 2025-11-06
//! 版本: 1.0.1
//! 
//! 修改记录:
//! - 2025-11-06: 初始化代码
//!
//! 描述: 
//! 这是一个Rust编写的插件化单机扫描工具，参考了FScan等知名开源项目
//! 支持端口扫描、Web扫描、系统信息扫描等功能。

mod logging;
mod global;
mod command;

use clap::Parser;
use tracing::*;
use anyhow::Result;

use crate::command::Commands;
use crate::{global::{SCAN_CONTEXT, ScanContext}, logging::init_logging};

#[derive(Parser, Debug)]
#[command(
    author = "拖延蟹@B1tf0rce",
    version = "1.0.1",
    about = "这是一个Rust编写的插件化单机扫描工具,开始参考了FScan等知名开源项目",
    long_about = "这是一个Rust编写的插件化单机扫描工具,开始参考了FScan等知名开源项目",
    disable_help_subcommand = true  // 禁用自动生成的 help 子命令
)]
pub struct Cli{
    /// 详细模式（For a flag that defaults to false and becomes true when present）
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    verbose: Option<bool>,
    
    #[command(subcommand)]
    command: Commands,
}

/// 初始扫描的全局变量
fn build_global_scan_context(cli: &Cli) -> ScanContext{
    let mut scan_context = ScanContext::default();

    if let Some(verbose) = cli.verbose{
        scan_context.verbose = verbose;
    }
    
    scan_context
}

/// 打印系统启动时的Banner
fn print_banner(){
        // 显示 Banner
    println!(r#"
    ██████╗ ██╗  ██╗   ███████╗ ██████╗ █████╗ ███╗   ██╗
    ██╔══██╗╚██╗██╔╝   ██╔════╝██╔════╝██╔══██╗████╗  ██║
    ██████╔╝ ╚███╔╝    ███████╗██║     ███████║██╔██╗ ██║
    ██╔══██╗ ██╔██╗    ╚════██║██║     ██╔══██║██║╚██╗██║
    ██║  ██║██╔╝ ██╗   ███████║╚██████╗██║  ██║██║ ╚████║
    ╚═╝  ╚═╝╚═╝  ╚═╝   ╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═══╝
    "#);
    println!("RXScan v1.0.1 - 单机扫描工具");
    println!("作者: 拖延蟹@B1tf0rce");
    println!("===============================================\n");
}

async fn _main()-> Result<()> {
    // 打印启动时的Banner
    print_banner();
    // 将启动参数初始化到对象
    let scan_cli = Cli::parse();
    // 初始化系统日志组件
    init_logging().await?;

    // 初始化全局配置元素
    let scan_context = build_global_scan_context(&scan_cli);

    SCAN_CONTEXT.set(scan_context).expect("初始化扫描全局变量失败");

    // 匹配命令执行
    scan_cli.command.execute().await
}

#[tokio::main]
async fn main() {
    // 工具主入口
    if let Err(error) = _main().await {
        error!("系统主函数初始化异常, 异常信息:{}", error);
        std::process::exit(1);
    }
}