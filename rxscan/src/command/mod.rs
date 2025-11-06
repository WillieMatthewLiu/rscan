//! 作者: 拖延蟹@B1tf0rce
//! 创建日期: 2025-11-06
//! 最后修改: 2025-11-06
//! 版本: 1.0.1
//! 
//! 修改记录:
//! - 2025-11-06: 初始化代码
//!
//! 描述: 
//! 子命令模块

mod port;
mod web;
mod sys;
mod help;

pub use port::PortArgs;
pub use web::WebArgs;
pub use sys::SysArgs;
pub use help::HelpArgs;

use clap::Subcommand;
use anyhow::Result;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 端口扫描
    Port(PortArgs),
    /// web扫描
    Web(WebArgs),
    /// 系统扫描
    Sys(SysArgs),
    /// 系统帮助
    Help(HelpArgs),
}

/// 定义Command的通用执行入口
impl Commands {
    pub async fn execute(&self) -> Result<()> {
        match self {
            Commands::Port(args) => port::execute(args).await,
            Commands::Web(args) => web::execute(args).await,
            Commands::Sys(args) => sys::execute(args).await,
            Commands::Help(args) => help::execute(args).await,
        }
    }
}