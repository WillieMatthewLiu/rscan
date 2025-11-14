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
//! APPFingerPrint<--->product_name: String
//!                    expression: Expression<--->keyword: String
//!                                               value: String
//!                                               operator: Operator<---> enum(!=,=,~=,==)

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::str::FromStr;
use tracing::*;

use crate::models::{Banner, VALID_KEYWORDS_LOWER};
use crate::parser::BooleanEvaluator;

/// 初始化表达式 Body="Swagger UI" || Body="/swagger-ui.css" || Body="swagger-ui-bundle.js" || Body="swagger-ui-standalone-preset.js"
static PARAM_REGEX: Lazy<Regex> = Lazy::new(|| {
    // Regex::new(r#"([a-zA-Z0-9]+)\s*(!=|=|~=|==)\s*"([^"\n]+)""#).expect("正则表达式编译失败")
    Regex::new(r#"(\w+)\s*([=!~]=?)\s*"([^"]*?)""#).expect("指纹匹配表达式编译失败")
});
/// 初始化表达式语法  ${1} || ${2}
///         let placeholder_re = Regex::new(r"\$\{\d+\}").unwrap();
///
static SYNTAX_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\$\{\d+\}").expect("指纹语法表达式编译失败"));

/// 初始化布尔语法表达式 (true & false) || true
static BOOL_SYNTAX_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"true|false|&|\||\(|\)").expect("指纹布尔语法表达式编译失败"));

/// 错误类型
#[derive(Debug, Clone)]
pub struct AppFingerError {
    message: String,
}

/// 实现对象的构建函数
impl AppFingerError {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

/// 实现标准输出的展示
impl std::fmt::Display for AppFingerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppFingerError = {}", self.message)
    }
}

/// 实现标准的Error Trait
impl Error for AppFingerError {}

/// 操作符枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Operator {
    NotEqual,   // !=
    Equal,      // =
    RegexEqual, // ~=
    SuperEqual, // ==
}

/// 将字符串转为操作符枚举
impl FromStr for Operator {
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

/// 实现标准输出的展示
impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::NotEqual => write!(f, "!="),
            Operator::Equal => write!(f, "="),
            Operator::RegexEqual => write!(f, "~="),
            Operator::SuperEqual => write!(f, "=="),
        }
    }
}

/// 应用特证判断的表达式对象
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Param {
    keyword: String,
    value: String,
    operator: Operator,
}

impl std::fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.operator {
            Operator::NotEqual => write!(f, "Param({},{},!=)", self.keyword, self.value),
            Operator::Equal => write!(f, "Param({},{},=)", self.keyword, self.value),
            Operator::RegexEqual => write!(f, "Param({},{},~=)", self.keyword, self.value),
            Operator::SuperEqual => write!(f, "Param({},{},==)", self.keyword, self.value),
        }
    }
}

/// Param构建
impl Param {
    fn new(expr: &str) -> Result<Self, AppFingerError> {
        // 正则解析
        let caps = PARAM_REGEX
            .captures(expr)
            .ok_or_else(|| AppFingerError::new(&format!("未知的参数表达式: {}", expr)))?;
        let keyword = caps.get(1).unwrap().as_str();
        let operator_str = caps.get(2).unwrap().as_str();
        let value_raw = caps.get(3).unwrap().as_str();

        // 验证关键字
        if !VALID_KEYWORDS_LOWER.contains(&keyword.to_lowercase()) {
            return Err(AppFingerError::new(&format!(
                "未知的HTTP Banner关键字: {}",
                keyword
            )));
        }
        // 字符串操作符转枚举
        let operator = Operator::from_str(operator_str)?;
        // 处理转义引号
        let value = value_raw.replace(r"\[quota\]", r#"""#);
        // 如果是正则表达式，验证其合法性 todo!()
        // if operator == Operator::RegexEqual {
        //     Regex::new(&value)
        //         .map_err(|e| {
        //             error!("操作符的正则表达式编译失败: {}", e);
        //             AppFingerError::new(&format!("操作符的正则表达式编译失败: {}", e))
        //         })?;
        // }

        Ok(Param {
            keyword: keyword.to_string(),
            value,
            operator,
        })
    }

    fn matches(&self, banner: &Banner) -> bool {
        let field_value = match banner.get_field(&self.keyword) {
            Some(v) => v,
            None => return false,
        };

        match self.operator {
            Operator::NotEqual => !field_value.contains(&self.value),
            Operator::Equal => field_value.contains(&self.value),
            Operator::RegexEqual => Regex::new(&self.value)
                .map(|re| re.is_match(field_value))
                .unwrap_or(false),
            Operator::SuperEqual => field_value == self.value,
        }
    }
}

// 对象指纹结构体  包含APP名称和指纹表达式
#[derive(Debug, Clone)]
pub struct FingerPrint {
    // 产品字典表行号
    product_id: usize,
    // 产品名称
    product_name: String,
    // 表达式参数切片
    param_slice: Vec<Param>,
    // 表达式原文
    value: String,
    // 表达式逻辑字符串
    // 示例: value = (body="test" || header="tt") && response="aaaa"
    //       expr  = (${1} || ${2}) && ${3}
    expr: String,
}

impl std::fmt::Display for FingerPrint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vec_string: String = self.param_slice.iter().map(|p| p.to_string()).collect();
        write!(
            f,
            "FingerPrint[{},{},{},{},{}]",
            self.product_id.to_string(),
            self.product_name,
            vec_string,
            self.value,
            self.expr
        )
    }
}

impl FingerPrint {
    pub fn new(
        product_id: &usize,
        product_name: &str,
        expression: &str,
    ) -> Result<Self, AppFingerError> {
        let value: String = expression.to_string();
        let mut expr_trimmed = value.trim().to_string();

        // 处理转义引号
        // info!("处理转义引号前的字符串: {}", expr_trimmed);
        expr_trimmed = expr_trimmed.replace(r#"\""#, r"\[quota\]");
        // warn!("处理转义引号后的字符串: {}", expr_trimmed);
        // debug!("需要验证的表达式: {}", expr_trimmed);
        // 验证--表达式
        Self::validate_expr(&expr_trimmed)?;

        // 提取参数
        let mut param_slice = Vec::new();
        let mut logical_expr = expr_trimmed.clone();

        for (i, cap) in PARAM_REGEX.captures_iter(&expr_trimmed).enumerate() {
            let full_match = cap.get(0).unwrap().as_str();
            trace!("表达式--第[{}]区，内容: {}", i + 1, full_match);
            let param = Param::new(full_match)?;
            param_slice.push(param);

            let placeholder = format!("${{{}}}", i + 1);
            logical_expr = logical_expr.replacen(full_match, &placeholder, 1);
        }

        debug!("表达式{} , 逻辑表达式: {}", value, logical_expr);
        // 验证--语法
        // Self::validate_syntax(&logical_expr)?;
        if let Err(ape) = Self::validate_syntax(&logical_expr) {
            error!("表达式[{}]语法验证错误, 异常信息: {}", value, ape);
        }
        Ok(FingerPrint {
            product_id: *product_id,
            product_name: product_name.to_string(),
            param_slice,        // 字段名改为param_slice
            value,              // 字段名改为value
            expr: logical_expr, // 字段名改为expr
        })
    }
    /// 验证表达式的合规性，如果表达式内容都替换了，就代表表达式合规
    fn validate_expr(expr: &str) -> Result<bool, AppFingerError> {
        // 移除所有命中的表达式
        let prune_result = PARAM_REGEX.replace_all(expr, "").to_string();
        // 移除所有逻辑字符和括号
        let result = prune_result.replace(|c: char| matches!(c, '&' | '|' | '(' | ')' | ' '), "");
        match result.trim() {
            "" => Ok(true),
            _ => {
                let unknown_chars = expr.to_string().replace(r"\[quota\]", r#"\""#);
                return Err(AppFingerError::new(&format!(
                    "未知表达式字符串: {}",
                    unknown_chars
                )));
            }
        }
    }
    /// 验证表达式语法
    fn validate_syntax(expr: &str) -> Result<bool, AppFingerError> {

        // 将${n}的表达式全部替换为true
        let bool_expr = SYNTAX_REGEX.replace_all(expr, "true").to_string();
        // 将true和运算符、括号替换成空
        let result = BOOL_SYNTAX_REGEX.replace_all(&bool_expr, "").to_string();
        // 验证表达式语法是否有错
        match result.trim() {
            "" => match BooleanEvaluator::eval(&bool_expr) {
                Ok(true) => Ok(true),
                Ok(false) => Ok(true),
                Err(msg) => Err(AppFingerError::new(msg.as_str())),
            },
            _ => Err(AppFingerError::new(&format!(
                "未知布尔表达式字符串: {}",
                result
            ))),
        }
    }

    /// 字符串匹配
    pub fn matches(&self, banner: &Banner) -> Option<String> {
        if self.expr_matches(banner) {
            let match_resutl = format!("{}_{}", self.product_id, &self.product_name);
            Some(match_resutl.clone())
        } else {
            None
        }
    }
    fn expr_matches(&self, banner: &Banner) -> bool {
        let mut expr = self.expr.clone();

        for (i, param) in self.param_slice.iter().enumerate() {
            let placeholder = format!("${{{}}}", i + 1);
            let result = param.matches(banner);
            expr = expr.replace(&placeholder, &result.to_string());
        }
        // Self::parse_bool_expression(&expr).unwrap_or(false)
        match BooleanEvaluator::eval(&expr) {
                Ok(true) => true,
                Ok(false) => false,
                Err(_msg) => false,
            }
    }
}

// 单元测试模块
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn validate_expr_test() {
        let expr = r#"Body="Swagger UI" || Body="/swagger-ui.css" || Body="swagger-ui-bundle.js" || Body="swagger-ui-standalone-preset.js""#;
        let prune_result = PARAM_REGEX.replace_all(expr, "");
        println!("替换后结果: {}", prune_result);
        // 移除所有参数
        // for cap in PARAM_REGEX.captures_iter(expr) {
        //     let full_match = cap.get(0).unwrap().as_str();
        //     println!("{}", full_match.to_string());
        //     test_expr = test_expr.replace(full_match, "");
        // }

        // 移除所有逻辑字符和括号
        let result = prune_result.replace(|c: char| matches!(c, '&' | '|' | '(' | ')' | ' '), "");
        println!("替换连接符后结果: {}", result);
        // if !result.is_empty() {
        //     let unknown_chars = test_expr.replace(r"\[quota\]", r#"\""#);
        //     println!("{}", unknown_chars);
        // }
    }
}
