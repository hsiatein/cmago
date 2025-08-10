use serde::{Deserialize, Serialize};
use crate::cmake_config::bin_config::BinConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct Bin {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub dependencies: Vec<String>, // 可能为空，所以使用默认值
}

impl Bin {
    pub fn new()->Self{
        Bin{name:"my_exec".to_string(),path:"./my_exec.cpp".to_string(),dependencies:vec!["my_lib".to_string()]}
    }
    pub fn to_bin_config(&self) -> BinConfig {
        BinConfig::new(self.name.as_str(),self.path.as_str())
    }
}