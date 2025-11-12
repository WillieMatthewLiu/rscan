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


use std::str::FromStr;
use std::error::Error;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::*;

use crate::models::{Banner, VALID_KEYWORDS_LOWER};

/// 初始化正则表达式
static PARAM_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"([a-zA-Z0-9]+)\s*(!=|=|~=|==)\s*"([^"\n]+)""#)
        .expect("正则表达式编译失败")
});


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
        write!(f, "应用指纹转换异常: {}", self.message)
    }
}

/// 实现标准的Error Trait
impl Error for AppFingerError {}

/// 操作符枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Operator {
    NotEqual,    // !=
    Equal,       // =
    RegexEqual,  // ~=
    SuperEqual,  // ==
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
            Operator::NotEqual => write!(f, "Param({},{},!=)",self.keyword,self.value),
            Operator::Equal => write!(f, "Param({},{},=)",self.keyword,self.value),
            Operator::RegexEqual => write!(f, "Param({},{},~=)",self.keyword,self.value),
            Operator::SuperEqual => write!(f, "Param({},{},==)",self.keyword,self.value),
        }
     }    
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

    fn matches(&self, banner: &Banner) -> bool {
        let field_value = match banner.get_field(&self.keyword) {
            Some(v) => v,
            None => return false,
        };
        
        match self.operator {
            Operator::NotEqual => !field_value.contains(&self.value),
            Operator::Equal => field_value.contains(&self.value),
            Operator::RegexEqual => {
                Regex::new(&self.value)
                    .map(|re| re.is_match(field_value))
                    .unwrap_or(false)
            }
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
        write!(f,"FingerPrint[{},{},{},{},{}]", self.product_id.to_string(),self.product_name,vec_string,self.value,self.expr)
    }
}

impl FingerPrint {
    pub fn new(product_id: &usize, product_name: &str, expression: &str) -> Result<Self, AppFingerError> {
        let value = expression.to_string();
        let mut expr_trimmed = expression.trim().to_string();
        
        // 处理转义引号
        expr_trimmed = expr_trimmed.replace(r#"\""#, r"\[quota\]");
        // debug!("需要验证的表达式: {}", expr_trimmed);
        // 验证--表达式
        Self::validate_chars(&expr_trimmed)?;
        
        // 提取参数
        let mut param_slice = Vec::new(); 
        let mut logical_expr = expr_trimmed.clone();
        
        for (i, cap) in PARAM_REGEX.captures_iter(&expr_trimmed).enumerate() {
            let full_match = cap.get(0).unwrap().as_str();
            trace!("表达式--第[{}]区，内容: {}", i+1,full_match);
            let param = Param::new(full_match)?;
            param_slice.push(param);
            
            let placeholder = format!("${{{}}}", i + 1);
            logical_expr = logical_expr.replacen(full_match, &placeholder, 1);
        }

        debug!("表达式{} , 逻辑表达式: {}",value, logical_expr);
        // 验证--语法 
        Self::validate_syntax(&logical_expr)?;
        
        Ok(FingerPrint {
            product_id: *product_id,
            product_name: product_name.to_string(),
            param_slice,    // 字段名改为param_slice
            value,          // 字段名改为value
            expr: logical_expr,  // 字段名改为expr
        })
    }
    /// 验证字符串
    fn validate_chars(expr: &str) -> Result<(), AppFingerError> {
        let mut test_expr = expr.to_string();
        
        // 移除所有参数
        for cap in PARAM_REGEX.captures_iter(expr) {
            let full_match = cap.get(0).unwrap().as_str();
            test_expr = test_expr.replace(full_match, "");
        }
        
        // 移除所有逻辑字符和括号
        test_expr = test_expr.replace(|c: char| matches!(c, '&' | '|' | '(' | ')' | ' '), "");
        
        if !test_expr.is_empty() {
            let unknown_chars = test_expr.replace(r"\[quota\]", r#"\""#);
            return Err(AppFingerError::new(&format!("未知字符串: {}", unknown_chars)));
        }
        
        Ok(())
    }
    /// 验证表达式语法
    fn validate_syntax(expr: &str) -> Result<(), AppFingerError> {
        let placeholder_re = Regex::new(r"\$\{\d+\}").unwrap();
        let test_expr = placeholder_re.replace_all(expr, "true").to_string();
        
        Self::parse_bool_expression(&test_expr)
            .map(|_| ())
            .map_err(|e| AppFingerError::new(&format!("表达式:{} 解析出现错误: {}", expr, e)))
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
        
        Self::parse_bool_expression(&expr).unwrap_or(false)
    }
    fn parse_bool_expression(expr: &str) -> Result<bool, AppFingerError> {
        let expr = expr.replace(' ', "");
        
        // 验证只有合法字符
        let valid_chars_re = Regex::new(r"^[truefalse&|()]+$").unwrap();
        if !valid_chars_re.is_match(&expr) {
            return Err(AppFingerError::new(&format!("解析布尔表达式时出错，未知的类型: {}", expr)));
        }
        
        Self::eval_bool_expression(&expr)
    }
    
    fn eval_bool_expression(expr: &str) -> Result<bool, AppFingerError> {
        if expr == "true" {
            return Ok(true);
        }
        if expr == "false" {
            return Ok(false);
        }
        
        let mut result: Option<bool> = None;
        let mut current_operator: Option<&str> = None;
        let mut i = 0;
        let chars: Vec<char> = expr.chars().collect();
        let len = chars.len();
        
        while i < len {
            match chars[i] {
                't' if i + 3 < len && &expr[i..i+4] == "true" => {
                    let value = true;
                    result = Some(Self::apply_operator(result, value, current_operator));
                    current_operator = None;
                    i += 4;
                }
                'f' if i + 4 < len && &expr[i..i+5] == "false" => {
                    let value = false;
                    result = Some(Self::apply_operator(result, value, current_operator));
                    current_operator = None;
                    i += 5;
                }
                '&' if i + 1 < len && chars[i+1] == '&' => {
                    current_operator = Some("&&");
                    i += 2;
                }
                '|' if i + 1 < len && chars[i+1] == '|' => {
                    current_operator = Some("||");
                    i += 2;
                }
                '(' => {
                    let end = Self::find_matching_parenthesis(&expr[i..])? + i;
                    let sub_expr = &expr[i+1..end];
                    let value = Self::eval_bool_expression(sub_expr)?;
                    result = Some(Self::apply_operator(result, value, current_operator));
                    current_operator = None;
                    i = end + 1;
                }
                ' ' => i += 1, // 跳过空格
                _ => return Err(AppFingerError::new(&format!("Unexpected character at position {}: {}", i, chars[i]))),
            }
        }
        
        result.ok_or_else(|| AppFingerError::new("Empty expression"))
    }
    
    fn apply_operator(current: Option<bool>, value: bool, operator: Option<&str>) -> bool {
        match (current, operator) {
            (None, _) => value, // 第一个值
            (Some(cur), Some("&&")) => cur && value,
            (Some(cur), Some("||")) => cur || value,
            (Some(cur), None) => cur, // 没有操作符，保持原值
            _ => false,
        }
    }
    
    fn find_matching_parenthesis(expr: &str) -> Result<usize, AppFingerError> {
        let mut balance = 0;
        for (i, c) in expr.chars().enumerate() {
            match c {
                '(' => balance += 1,
                ')' => {
                    balance -= 1;
                    if balance == 0 {
                        return Ok(i);
                    }
                }
                _ => {}
            }
        }
        Err(AppFingerError::new("Unmatched parentheses"))
    }
}