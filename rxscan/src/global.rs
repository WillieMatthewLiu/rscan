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
