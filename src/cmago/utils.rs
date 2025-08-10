use std::path::{Path, PathBuf};
use std::fs;
use crate::cmake_config::cmake_config::CmakeConfig;
use crate::cmake_generator::command::CMakeLists;
use crate::parser::parser::parse;

pub fn find_cmago() -> Option<PathBuf> {
    // 从当前目录开始
    let mut dir = std::env::current_dir().ok()?;

    loop {
        let candidate = dir.join("Cmago.toml");

        if candidate.exists() && candidate.is_file() {
            return Some(dir); // 找到就返回所在目录
        }

        // 如果到根目录了还没找到，就退出
        if !dir.pop() {
            break;
        }
    }

    None
}

pub fn get_cmake_config(path:&Path) -> CmakeConfig {
    let path = path.join(Path::new("Cmago.toml"));
    let path = path.as_path();
    if !path.exists() {
        panic!("Cmago file does not exist");
    }
    let path_str=path.as_os_str().to_str().unwrap();
    parse(path_str)
}
