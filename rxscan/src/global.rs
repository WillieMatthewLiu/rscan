//! 作者: 拖延蟹@B1tf0rce
//! 创建日期: 2025-11-06
//! 最后修改: 2025-11-06
//! 版本: 1.0.1
//! 
//! 修改记录:
//! - 2025-11-06: 初始化代码
//!
//! 描述: 
//! 接受全局参数

use std::sync::OnceLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanContext {
    pub verbose: bool,
}

impl Default for ScanContext{
    fn default() -> Self{
        Self{
            verbose: false,
        }
    }
}

pub static SCAN_CONTEXT: OnceLock<ScanContext> = OnceLock::new();
