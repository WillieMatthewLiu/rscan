use anyhow::{Context, Result};
use time::{format_description, UtcOffset};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::{EnvFilter, Layer};
use tracing_subscriber::layer::SubscriberExt;  
use tracing_subscriber::util::SubscriberInitExt;  

pub async fn init_logging() -> Result<()> {
    LogTracer::init().context("Failed to initialize log compatibility layer")?;

    let offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // 命令行交互环境专用配置
    let formatting_layer = tracing_subscriber::fmt::layer()
        .compact()  // 紧凑格式更适合命令行
        .with_ansi(true)  // 命令行支持颜色
         // 隐藏目标信息，更简洁
        .with_timer(OffsetTime::new(
            offset,
            format_description::parse("[hour]:[minute]:[second]")
                .context("Failed to parse time format")?,
        ))
        .with_file(true)             // 显示文件名
        .with_line_number(true)      // 显示行号
        .with_thread_ids(true)
        .with_target(false);       // 显示线程ID

    tracing_subscriber::registry()
        .with(formatting_layer)
        .with(env_filter)
        .init();

    Ok(())
}