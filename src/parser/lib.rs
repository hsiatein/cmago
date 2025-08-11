use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Lib {
    pub name: String,
    pub path: String,
    #[serde(default = "default_lib_type")]
    pub lib_type: String,
    #[serde(default)]
    pub dependencies: Vec<String>, // 可能为空，所以使用默认值
}

impl Lib {
    pub fn new()->Self{
        Lib{name:"my_lib".to_string(), path:"./my_lib/".to_string(),lib_type:"static".to_string(),dependencies:vec![]}
    }
}

fn default_lib_type() -> String {
    "static".to_string()
}