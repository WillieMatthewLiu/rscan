//! 作者: 拖延蟹@B1tf0rce
//! 创建日期: 2025-11-06
//! 最后修改: 2025-11-06
//! 版本: 1.0.1
//! 
//! 修改记录:
//! - 2025-11-06: 初始化代码
//!
//! 描述: 
//! 日志处理的通用方法

use anyhow::{Context, Result};
use time::{format_description, UtcOffset};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;  
use tracing_subscriber::util::SubscriberInitExt;  
use tracing_appender::non_blocking::WorkerGuard;
use once_cell::sync::OnceCell;

static _FILE_GUARD: OnceCell<WorkerGuard> = OnceCell::new();

pub async fn init_logging() -> Result<()> {
    // 桥接 log crate（让依赖库的 log 调用也能被 tracing 捕获）
    LogTracer::init().context("Failed to initialize log compatibility layer")?;

    let offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    
    //日志过滤
    let env_filter = EnvFilter::new("info");

    // 命令行交互环境专用配置
    let console_layer = tracing_subscriber::fmt::layer()
        .compact()  // 紧凑格式更适合命令行
        .with_ansi(true)  // 命令行支持颜色
        .with_timer(OffsetTime::new(
            offset,
            format_description::parse("[hour repr:24]:[minute]:[second].[subsecond digits:3]")
                .context("解析时间格式异常")?,
        ))
        .with_file(false)             // 显示文件名
        .with_line_number(false)      // 显示行号
        .with_thread_ids(true)       // 显示线程ID
        .with_target(false);   // 显示依赖库信息

    // 日志文件配置
    // 文件层（使用非阻塞写入）
    let file_appender = tracing_appender::rolling::daily("logs", "rxscan.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    // 将guard存储到全局变量中
    _FILE_GUARD.set(_guard).expect("设置日志输出全局WorkerGuard对象失败!");

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)  // 文件不支持颜色
        .compact()
        .with_timer(OffsetTime::new(
            offset,
            format_description::parse("[hour repr:24]:[minute]:[second].[subsecond digits:3]")
                .context("解析时间格式异常")?,
        ))
        .with_file(false)             // 显示文件名
        .with_line_number(false)      // 显示行号
        .with_thread_ids(true)       // 显示线程ID
        .with_target(false);   // 显示依赖库信息

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .with(env_filter)
        .init();

    Ok(())
}