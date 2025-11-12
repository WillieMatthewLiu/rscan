//! 作者: 拖延蟹@B1tf0rce
//! 创建日期: 2025-11-10
//! 最后修改: 2025-11-10
//! 版本: 1.0.1
//! 
//! 修改记录:
//! - 2025-11-10: 初始化代码
//!
//! 描述: 
//! 创建APP指纹库, 爆破用字典等
//! 
use once_cell::sync::OnceCell;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use tracing::*;


use crate::models::Banner;

// pub mod app_fingerprint;
// pub use crate::bullet::app_fingerprint::{AppFingerError, Expression,FingerPrint};

pub mod app_fp;
pub use crate::bullet::app_fp::{AppFingerError, FingerPrint};

/// 定义指纹库
type FingerPrintDB = Vec<FingerPrint>;
pub static GLOBAL_FINGERPRINTS: OnceCell<FingerPrintDB> = OnceCell::new();

/// 公共API - 使用完整路径
pub fn init_database_with_path(path: &std::path::Path) -> Result<(), AppFingerError> {
    let file = File::open(path)
        .map_err(|e| AppFingerError::new(&format!("未找到指纹库文件 {}: {}", path.display(), e)))?;
    
    let db = init_database_reader(BufReader::new(file))?;
    
    GLOBAL_FINGERPRINTS.set(db)
        .map_err(|_| AppFingerError::new("全局APP指纹库已经加载"))
}

// 内部初始化逻辑
fn init_database_reader<R: Read>(reader: R) -> Result<FingerPrintDB, AppFingerError> {
    let buf_reader = BufReader::new(reader);
    let mut db = Vec::new();
    let mut last_error: Option<AppFingerError> = None;
    let mut line_count = 0;
    let mut success_count = 0;
    
    for (line_num, line) in buf_reader.lines().enumerate() {
        line_count += 1;
        let line = line.map_err(|e| AppFingerError::new(&format!("IO error at line {}: {}", line_num + 1, e)))?;
        let line = line.trim();
        
        if line.is_empty() || line.starts_with('#') {
            continue; // 跳过空行和注释
        }
        
        let parts: Vec<&str> = line.splitn(2, '\t').collect();
        if parts.len() != 2 {
            if line_num > 0 {
                last_error = Some(AppFingerError::new(&format!("Invalid line format at line {}: {}", line_num + 1, line)));
            }
            continue;
        }
        
        match add_fingerprint(&mut db, &line_count,parts[0], parts[1]) {
            Ok(_) => success_count += 1,
            Err(e) => {
                warn!("解析APP指纹库文件异常，异常行号： {}，异常内容: {}", line_num + 1, e);
                if line_num > 0 {
                    last_error = Some(e);
                }
            }
        }
    }
    
    println!("Fingerprint database loaded: {} fingerprints from {} lines", success_count, line_count);
    
    if let Some(err) = last_error {
        Err(err)
    } else if success_count == 0 {
        Err(AppFingerError::new("No valid fingerprints found in database"))
    } else {
        Ok(db)
    }
}

/// 内部添加指纹逻辑
fn add_fingerprint(db: &mut FingerPrintDB,product_id: &i32, product_name: &str, expression: &str) -> Result<(), AppFingerError> {
    let fingerprint = FingerPrint::new(product_id,product_name, expression)?;
    db.push(fingerprint);
    Ok(())
}

/// 搜索功能
pub fn search(banner: &Banner) -> Vec<String> {
    GLOBAL_FINGERPRINTS.get()
        .map(|db| {
            let mut products = Vec::new();
            let mut seen_products = HashSet::new();
            
            for fingerprint in db {
                if let Some(product) = fingerprint.matches(banner) {
                    if !seen_products.contains(&product) {
                        seen_products.insert(product.clone());
                        products.push(product);
                    }
                }
            }
            
            products
        })
        .unwrap_or_else(|| {
            eprintln!("Warning: Fingerprint database not initialized. Call init_database() first.");
            Vec::new()
        })
}


/// 状态查询
pub fn is_initialized() -> bool {
    GLOBAL_FINGERPRINTS.get().is_some()
}

/// 指纹库大小
pub fn get_fingerprint_count() -> usize {
    // let global_fp_size = GLOBAL_FINGERPRINTS.get().map(|db| db.len()).unwrap_or(0);
    // if global_fp_size > 0{
    //     warn!(format!("{}",GLOBAL_FINGERPRINTS.get(0)));
    // }
    if let Some(first_fp) = GLOBAL_FINGERPRINTS.get().and_then(|db| db.first()) {
        debug!("First fingerprint: {}", first_fp);
    } else {
        debug!("No fingerprints available");
    }
    GLOBAL_FINGERPRINTS.get().map(|db| db.len()).unwrap_or(0)    
}
