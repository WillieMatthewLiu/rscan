//! 作者: 拖延蟹@B1tf0rce
//! 创建日期: 2025-11-10
//! 最后修改: 2025-11-10
//! 版本: 1.0.1
//! 
//! 修改记录:
//! - 2025-11-10: 初始化代码
//!
//! 描述: 
//! http扫描结果的对象

use once_cell::sync::Lazy;

pub static VALID_KEYWORDS_LOWER: Lazy<Vec<String>> = Lazy::new(|| {
    vec![
        "title".to_string(),
        "header".to_string(),
        "body".to_string(),
        "response".to_string(),
        "protocol".to_string(),
        "cert".to_string(),
        "port".to_string(),
        "hash".to_string(),
        "icon".to_string(),
    ]
});


#[derive(Debug, Default, Clone)]
pub struct Banner {
    pub title: Option<String>,
    pub header: Option<String>,
    pub body: Option<String>,
    pub response: Option<String>,
    pub protocol: Option<String>,
    pub cert: Option<String>,
    pub port: Option<String>,
    pub hash: Option<String>,
    pub icon: Option<String>,
}

impl Banner {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn get_field(&self, field_name: &str) -> Option<&str> {
        match field_name {
            "Title" => self.title.as_deref(),
            "Header" => self.header.as_deref(),
            "Body" => self.body.as_deref(),
            "Response" => self.response.as_deref(),
            "Protocol" => self.protocol.as_deref(),
            "Cert" => self.cert.as_deref(),
            "Port" =>  self.port.as_deref(),
            "Hash" => self.hash.as_deref(),
            "Icon" => self.icon.as_deref(),
            _ => None,
        }
    }
}