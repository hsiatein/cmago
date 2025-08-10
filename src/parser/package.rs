use serde::{Deserialize, Serialize};
use crate::cmake_config::cmake_config::CmakeConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    pub project: String,
    pub version: String,
    #[serde(rename = "cmake_minimum_required")]
    pub cmake_minimum_required: String,
    #[serde(rename = "cpp_standard")]
    pub cpp_standard: String,
    #[serde(rename = "external_library_path")]
    pub external_library_path: String,
}

impl Package {
    pub fn new()->Self{
        Package{project:"my_project".to_string(),version:"0.1.0".to_string(),cmake_minimum_required:"3.12".to_string(),cpp_standard:"17".to_string(),external_library_path:"./external/".to_string()}
    }
    pub fn to_cmake_config(&self) -> CmakeConfig {
        CmakeConfig::new(self.project.clone(),self.version.clone(),self.cmake_minimum_required.clone(),self.cpp_standard.clone(),self.external_library_path.clone())
    }
}