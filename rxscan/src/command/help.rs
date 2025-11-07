//! 作者: 拖延蟹@B1tf0rce
//! 创建日期: 2025-11-06
//! 最后修改: 2025-11-06
//! 版本: 1.0.1
//! 
//! 修改记录:
//! - 2025-11-06: 初始化代码
//!
//! 描述: 
//! 帮助命令模块

use clap::Args;
use tracing::*;
use anyhow::Result;

#[derive(Args, Debug)]
pub struct HelpArgs {
    /// 查看具体命令的帮助
    pub command: Option<String>,
}

pub async fn execute(args: &HelpArgs) -> Result<()> {
    info!("显示帮助信息");
    
    println!("RXScan 帮助信息");
    println!("==================");
    println!();
    println!("可用命令:");
    println!("  port <IPs>    - 端口扫描");
    println!("  web <URL>     - Web应用扫描");
    println!("  sys <IPs>     - 系统信息扫描");
    println!("  help [命令]    - 显示帮助信息");
    println!();
    println!("示例:");
    println!("  rxscan port 192.168.1.1 -p 80,443,8080");
    println!("  rxscan web https://example.com");
    println!("  rxscan sys 192.168.1.1 --scan-type full");
    println!();
    
    if let Some(cmd) = &args.command {
        show_command_help(cmd);
    }
    
    Ok(())
}

fn show_command_help(cmd: &str) {
    match cmd.to_lowercase().as_str() {
        "port" => {
            println!("端口扫描命令帮助:");
            println!("  rxscan port <目标> [选项]");
            println!();
            println!("参数:");
            println!("  <目标>        目标IP地址或网段，如 192.168.1.1 或 192.168.1.0/24");
            println!();
            println!("选项:");
            println!("  -p, --ports   端口范围，默认: 1-65535");
            println!("                支持格式: 80,443,8080 或 1-1000 或 80,443,1000-2000");
        },
        "web" => {
            println!("Web扫描命令帮助:");
            println!("  rxscan web [选项]");
            println!();
            println!("选项:");
            println!("  -u, --url     目标URL");
            println!("  -d, --depth   扫描深度，默认: 3");
        },
        "sys" => {
            println!("系统扫描命令帮助:");
            println!("  rxscan sys <目标> [选项]");
            println!();
            println!("参数:");
            println!("  <目标>        目标IP地址");
            println!();
            println!("选项:");
            println!("  -s, --scan-type  扫描类型，默认: basic");
            println!("                   可选: basic, full, os, service");
        },
        _ => {
            println!("未知命令: {}", cmd);
        }
    }
}