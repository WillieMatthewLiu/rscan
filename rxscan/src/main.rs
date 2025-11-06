mod logging;
mod global;

use clap::{Parser, Subcommand, Args};
use tracing::*;
use anyhow::Result;

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

#[derive(Subcommand, Debug)]
pub enum Commands{
    // 端口扫描
    Port(PortArgs),
    // web扫描
    Web(WebArgs),
    // 系统扫描
    Sys(SysArgs),
    // 系统帮助
    Help(HelpArgs),

}

/// 端口扫描的参数
#[derive(Args, Debug)]
pub struct PortArgs{
    // 目标主机或者网段
    target: String,
        
    // 端口范围
    #[arg(short, long, default_value = "1-65535")]
    ports: String,
}

/// Web扫描的参数
#[derive(Args, Debug)]
pub struct WebArgs{
    
}

/// 主机信息扫描的参数
#[derive(Args, Debug)]
pub struct SysArgs{
    
}

/// 工具帮助信息的参数
#[derive(Args, Debug)]
pub struct HelpArgs{
    
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
    ██████╗  ██╗  ██╗███████╗ █████╗ ███╗   ██╗
    ██╔══██╗ ╚██╗██╔╝██╔════╝██╔══██╗████╗  ██║
    ██████╔╝  ╚███╔╝ ███████╗███████║██╔██╗ ██║
    ██╔══██╗  ██╔██╗ ╚════██║██╔══██║██║╚██╗██║
    ██║  ██║ ██╔╝ ██╗███████║██║  ██║██║ ╚████║
    ╚═╝  ╚═╝ ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═══╝
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
    match &scan_cli.command {
        Commands::Port(_args) => {
            info!("开始端口扫描: target={}, ports={}", _args.target, _args.ports);
            Ok(())
        },
        Commands::Web(_args) => {
            info!("开始Web扫描");
            Ok(())
        },
        Commands::Sys(_args) => {
            info!("开始主机信息扫描");
            Ok(())
        },
        Commands::Help(_args) => {
            Ok(())
        },
        _ => {
            // 确保有返回值
            Ok(())
        },
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