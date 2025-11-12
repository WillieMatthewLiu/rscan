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

use once_cell::sync::OnceCell;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use tracing::*;

use crate::models::Banner;

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

    GLOBAL_FINGERPRINTS
        .set(db)
        .map_err(|_| AppFingerError::new("全局APP指纹库已经加载"))
}

// 内部初始化逻辑
fn init_database_reader<R: Read>(reader: R) -> Result<FingerPrintDB, AppFingerError> {
    let buf_reader = BufReader::new(reader);
    let start_time = std::time::Instant::now();

    // 一次性读取所有行，封装为(行号, 内容)的元组
    let lines: Vec<(usize, String)> = buf_reader
        .lines()
        .enumerate()
        .map(|(line_num, line_result)| {
            let content =
                line_result.map_err(|e| AppFingerError::new(&format!("文件读取异常: {}", e)))?;
            Ok((line_num + 1, content)) // 行号从1开始
        })
        .collect::<Result<Vec<(usize, String)>, AppFingerError>>()?;

    // 使用map-reduce模式，避免频繁加锁
    let (db, errors): (Vec<_>, Vec<_>) = lines
        .par_iter()
        .filter_map(|(line_num, content)| {
            let line = content.trim();

            // 跳过空行和注释
            if line.is_empty() || line.starts_with('#') {
                return None;
            }

            // 使用split_once更高效
            let (product_name, expression) = line.split_once('\t')?;

            match parse_fingerprint_line(*line_num, product_name, expression) {
                Ok(fp) => {
                    Some(Ok(fp))
                }
                Err(e) => Some(Err(e)),
            }
        })
        .partition(|result| result.is_ok());

    // 提取结果
    let db: FingerPrintDB = db.into_iter().filter_map(Result::ok).collect();

    let errors: Vec<AppFingerError> = errors.into_iter().filter_map(Result::err).collect();

    let elapsed_time = start_time.elapsed();
    
    info!("指纹库解析耗时: {}毫秒", elapsed_time.as_millis());
    info!(
        "成功识别加载的指纹有[{}]个",
        db.len()
    );

    if let Some(last_err) = errors.last() {
        Err(AppFingerError::new(
            format!(
                "加载过程中错误的指纹个数[{}],最后的错误信息: {}",
                errors.len(),
                last_err
            )
            .as_str(),
        ))
    } else if db.len() == 0 {
        Err(AppFingerError::new("未成功识别加载任何应用指纹"))
    } else {
        Ok(db)
    }
}

/// 内部添加指纹逻辑
fn parse_fingerprint_line(
    line_num: usize,
    product_name: &str,
    expression: &str,
) -> Result<FingerPrint, AppFingerError> {
    // FingerPrint::new(&product_id, product_name, expression)
    //     .map_err(|e| AppFingerError::new(&format!("在[{}]行解析异常: {}", line_num, e)))
    FingerPrint::new(&line_num, product_name, expression)
}

/// 搜索功能
pub fn search(banner: &Banner) -> Vec<String> {
    GLOBAL_FINGERPRINTS
        .get()
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
    // if let Some(first_fp) = GLOBAL_FINGERPRINTS.get().and_then(|db| db.first()) {
    //     debug!("抽样第一个指纹打印: {}", first_fp);
    // }
    GLOBAL_FINGERPRINTS.get().map(|db| db.len()).unwrap_or(0)
}
