use crate::cmake_config::{cmake_config::CmakeConfig,bin_config::BinConfig,lib_config::LibConfig};
use serde::{Deserialize, Serialize};
use std::fs;
use crate::cmake_config::has_dependencies::HasDependencies;
use crate::parser::bin::Bin;
use crate::parser::deps::Deps;
use crate::parser::lib::Lib;
use crate::parser::package::Package;
use crate::parser::tests::Tests;

#[derive(Debug, Deserialize,Serialize)]
pub struct CmagoToml {
    #[serde(rename = "package")]
    pub package: Package,

    #[serde(rename = "bin", default)]
    pub bins: Vec<Bin>,

    #[serde(rename = "lib", default)]
    pub libs: Vec<Lib>,

    #[serde(rename = "tests")]
    pub tests: Tests,

    #[serde(rename = "dependencies")]
    pub deps: Deps, // 用 HashMap 来处理外部库依赖
}

impl CmagoToml {
    pub fn new()->Self{
        CmagoToml{package:Package::new(),bins:vec![Bin::new()],libs:vec![Lib::new()],tests:Tests::new(""),deps:Deps::new()}
    }
    // 从文件路径读取并解析 TOML 文件
    pub fn from_path(path: &str) -> CmagoToml {
        // 读取 TOML 配置文件内容
        let toml_content = fs::read_to_string(path).expect(&format!("Cannot read file: {}", path));

        // 解析 TOML 内容为 CmagoToml 结构体
        let config: CmagoToml = toml::de::from_str(&toml_content).expect(&format!("parse failed: {}", path));
        config
    }

    pub fn to_cmake_config(&self) -> CmakeConfig {
        let mut cmake_config = self.package.to_cmake_config();
        cmake_config.set_deps(self.deps.to_external_configs(self.package.external_library_path.as_str()));

        for lib in &self.libs {
            if cmake_config.has_lib(lib.name.as_str()){
                continue;
            }
            self.register_lib(lib, &mut cmake_config);
        }
        for bin in &self.bins {
            self.register_bin(bin, &mut cmake_config);
        }
        cmake_config.tests.set_path(self.tests.path.as_str());

        cmake_config
    }

    fn get_lib(&self, name:&str)->&Lib {
        self.libs.iter().find(|lib| lib.name == name).expect(&format!("lib <{name}> not found"))
    }

    fn register_lib(&self, lib:&Lib, cmake_config: &mut CmakeConfig)->() {
        let mut lib_config = LibConfig::new(lib.name.as_str(),lib.path.as_str());
        for dep in &lib.dependencies{
            if !cmake_config.has_lib(dep.as_str()) {
                self.register_lib(self.get_lib(dep), cmake_config);
            }
            lib_config.add_dependency(&cmake_config.get_lib(dep));
        }
        cmake_config.add_lib(lib_config);
    }
    fn register_bin(&self, bin:&Bin, cmake_config: &mut CmakeConfig)->() {
        let mut bin_config = BinConfig::new(bin.name.as_str(),bin.path.as_str());
        for dep in &bin.dependencies{
            bin_config.add_dependency(&cmake_config.get_lib(dep));
        }
        cmake_config.add_bin(bin_config);
    }
    
    pub fn to_string(&self) -> String {
        toml::to_string(&self).unwrap()
    }
}


pub fn parse(path: &str)-> CmakeConfig {
    let cmago_toml = CmagoToml::from_path(path);
    cmago_toml.to_cmake_config()
}





#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_path() {
        let cmago_toml = CmagoToml::from_path(r"C:\Users\30236\Desktop\programming\cmago\assets\cmago.toml");
        println!("{cmago_toml:?}");
    }
}