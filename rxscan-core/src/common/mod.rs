//! 作者: 拖延蟹@B1tf0rce
//! 创建日期: 2025-11-14
//! 最后修改: 2025-11-14
//! 版本: 1.0.1
//!
//! 修改记录:
//! - 2025-11-14: 异步任务资源池

mod async_task;
pub use crate::common::async_task::{AsyncTaskPool, TaskPoolConfig};