use serde::{Deserialize, Serialize};
use crate::cmake_config::tests_config::TestsConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tests {
    pub path: String,
}

impl Tests {
    pub fn new(path: &str) -> Self {
        Tests{path:path.to_string()}
    }
    
    pub fn to_tests_config(&self) -> TestsConfig {
        TestsConfig::new()
    }
}