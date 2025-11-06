//! 作者: 拖延蟹@B1tf0rce
//! 创建日期: 2025-11-06
//! 最后修改: 2025-11-06
//! 版本: 1.0.1
//! 
//! 修改记录:
//! - 2025-11-06: 初始化代码
//!
//! 描述: 
//! 端口扫描模块

use clap::Args;
use tracing::*;
use anyhow::{Ok, Result};

#[derive(Args, Debug)]
pub struct PortArgs {
    /// 目标主机或者网段
    pub target: String,
        
    /// 端口范围
    #[arg(short, long, default_value = "1-65535")]
    pub ports: String,

    // 设置用户交互
    // 如: rxscan port 192.168.1.1 --interactive ，此时interactive会设置为true
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    interactive: bool,
}

pub async fn execute(args: &PortArgs) -> Result<()> {
    info!("开始端口扫描: target={}, ports={}", args.target, args.ports);
    
    // 这里添加实际的端口扫描逻辑
    println!("开始端口扫描...");
    println!("目标: {}", args.target);
    println!("端口范围: {}", args.ports);
    
    // 示例：解析端口范围
    let ports = parse_ports(&args.ports)?;
    println!("待扫描端口数量: {}", ports.len());
    
    // 实际的扫描逻辑将在这里实现
    // 比如：多线程扫描、端口状态检测等
      if args.interactive {
        run_interactive().await?;
    }
    info!("端口扫描完成");
    Ok(())
}

/// 解析端口字符串，支持格式：80,443,8080 或 1-1000 或 80,443,1000-2000
fn parse_ports(ports_str: &str) -> Result<Vec<u16>> {
    let mut ports = Vec::new();
    
    for part in ports_str.split(',') {
        if part.contains('-') {
            // 处理端口范围，如 1-1000
            let range_parts: Vec<&str> = part.split('-').collect();
            if range_parts.len() == 2 {
                let start = range_parts[0].parse::<u16>()?;
                let end = range_parts[1].parse::<u16>()?;
                for port in start..=end {
                    ports.push(port);
                }
            }
        } else {
            // 处理单个端口
            let port = part.parse::<u16>()?;
            ports.push(port);
        }
    }
    
    ports.sort();
    ports.dedup();
    Ok(ports)
}

/// 扫描完成后用户交互式操作
async fn run_interactive() -> Result<()> {
    loop {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ports() {
        let ports = parse_ports("80,443,8080").unwrap();
        assert_eq!(ports, vec![80, 443, 8080]);
        
        let ports = parse_ports("1-3").unwrap();
        assert_eq!(ports, vec![1, 2, 3]);
        
        let ports = parse_ports("80,443,1000-1002").unwrap();
        assert_eq!(ports, vec![80, 443, 1000, 1001, 1002]);
    }
}