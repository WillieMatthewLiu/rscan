//! 作者: 拖延蟹@B1tf0rce
//! 创建日期: 2025-11-14
//! 最后修改: 2025-11-14
//! 版本: 1.0.1
//!
//! 修改记录:
//! - 2025-11-14: 布尔表达式解析

use pest_derive::Parser;
use pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "parser/bool_gram.pest"]
pub struct BooleanParser;

pub struct BooleanEvaluator;

impl BooleanEvaluator {
    /// 解析并计算布尔表达式
    pub fn eval(expr: &str) -> Result<bool, String> {
        let mut pairs = BooleanParser::parse(Rule::input, expr)
            .map_err(|e| format!("语法错误: {}", e))?;
        
        let expression_pair = pairs.next().unwrap();
        Self::eval_expression(expression_pair.into_inner().next().unwrap())
    }
    
    /// 递归计算表达式
    fn eval_expression(pair: Pair<Rule>) -> Result<bool, String> {
        match pair.as_rule() {
            Rule::expression => Self::eval_logical_or(pair.into_inner().next().unwrap()),
            Rule::logical_or => Self::eval_logical_or(pair),
            Rule::logical_and => Self::eval_logical_and(pair),
            Rule::term => Self::eval_term(pair),
            Rule::boolean => Self::eval_boolean(pair),
            _ => Err(format!("未知规则: {:?}", pair.as_rule())),
        }
    }
    
    /// 计算逻辑或运算
    fn eval_logical_or(pair: Pair<Rule>) -> Result<bool, String> {
        let mut inner = pair.into_inner();
        let mut result = Self::eval_logical_and(inner.next().unwrap())?;
        
        // 处理连续的 || 运算
        while let Some(_op) = inner.next() {
            let right = Self::eval_logical_and(inner.next().unwrap())?;
            result = result || right;
        }
        
        Ok(result)
    }
    
    /// 计算逻辑与运算
    fn eval_logical_and(pair: Pair<Rule>) -> Result<bool, String> {
        let mut inner = pair.into_inner();
        let mut result = Self::eval_term(inner.next().unwrap())?;
        
        // 处理连续的 && 运算
        while let Some(_op) = inner.next() {
            let right = Self::eval_term(inner.next().unwrap())?;
            result = result && right;
        }
        
        Ok(result)
    }
    
    /// 计算基本项
    fn eval_term(pair: Pair<Rule>) -> Result<bool, String> {
        let mut inner = pair.into_inner();
        let first = inner.next().unwrap();
        
        match first.as_rule() {
            Rule::boolean => Self::eval_boolean(first),
            Rule::expression => Self::eval_expression(first),
            Rule::not_operator => {
                let operand = Self::eval_term(inner.next().unwrap())?;
                Ok(!operand)
            }
            _ => Err(format!("无效的项: {:?}", first.as_rule())),
        }
    }
    
    /// 计算布尔值
    fn eval_boolean(pair: Pair<Rule>) -> Result<bool, String> {
        Ok(match pair.as_str() {
            "true" => true,
            "false" => false,
            _ => return Err(format!("无效的布尔值: {}", pair.as_str())),
        })
    }
}