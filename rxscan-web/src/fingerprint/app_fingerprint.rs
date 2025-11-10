//! 作者: 拖延蟹@B1tf0rce
//! 创建日期: 2025-11-10
//! 最后修改: 2025-11-10
//! 版本: 1.0.1
//! 
//! 修改记录:
//! - 2025-11-10: 初始化代码
//!
//! 描述: 
//! 创建APP指纹库
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use regex::Regex;
use crate::models::{Banner, VALID_KEYWORDS_LOWER};

/// 初始化正则表达式
static PARAM_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"([a-zA-Z0-9]+)\s*(!=|=|~=|==)\s*"([^"\n]+)""#)
        .expect("正则表达式编译失败")
});


/// 错误类型
#[derive(Debug)]
pub struct AppFingerError {
    message: String,
}

/// 实现对象的构建函数
impl AppFingerError {
    fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

/// 实现标准输出的展示
impl std::fmt::Display for AppFingerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "应用指纹转换异常: {}", self.message)
    }
}

/// 实现标准的Error Trait
impl std::error::Error for AppFingerError {}

/// 操作符枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Operator {
    NotEqual,    // !=
    Equal,       // =
    RegexEqual,  // ~=
    SuperEqual,  // ==
}

/// 将字符串转为操作符枚举
impl std::str::FromStr for Operator {
    type Err = AppFingerError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "!=" => Ok(Operator::NotEqual),
            "=" => Ok(Operator::Equal),
            "~=" => Ok(Operator::RegexEqual),
            "==" => Ok(Operator::SuperEqual),
            _ => Err(AppFingerError::new(&format!("未知的表达式类型: {}", s))),
        }
    }
}

/// 参数结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Param {
    keyword: String,
    value: String,
    operator: Operator,
}

/// Param构建
impl Param{
    fn new(expr: &str) -> Result<Self,AppFingerError>{
        // 正则解析
        let caps =PARAM_REGEX.captures(expr)
            .ok_or_else(|| AppFingerError::new(&format!("未知的参数表达式: {}", expr)))?;
        let keyword = caps.get(1).unwrap().as_str();
        let operator_str = caps.get(2).unwrap().as_str();
        let value_raw = caps.get(3).unwrap().as_str();

        // 验证关键字
        if !VALID_KEYWORDS_LOWER.contains(&keyword.to_lowercase()) {
            return Err(AppFingerError::new(&format!("未知的HTTP Banner关键字: {}", keyword)));
        }
        
        // 字符串操作符转枚举
        let operator = Operator::from_str(operator_str)?;
        
        // 处理转义引号
        let value = value_raw.replace(r"\[quota\]", r#"""#);

        // 如果是正则表达式，验证其合法性
        if operator == Operator::RegexEqual {
            Regex::new(&value)
                .map_err(|e| AppFingerError::new(&format!("操作符的正则表达式编译失败: {}", e)))?;
        }
        
        Ok(Param {
             keyword: keyword.to_string(),
            value,
            operator,
        })
    }
}


/// 表达式结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expression {
    // 表达式参数切片
    param_slice: Vec<Param>,
    // 表达式原文
    value: String,
    // 表达式逻辑字符串
    // 示例: value = (body="test" || header="tt") && response="aaaa"
    //       expr  = (${1} || ${2}) && ${3}
    expr: String,
}