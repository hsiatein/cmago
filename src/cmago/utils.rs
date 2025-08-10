use std::path::{Path, PathBuf};
use std::{fs, io};
use std::fs::File;
use std::io::Write;
use crate::cmake_config::cmake_config::CmakeConfig;
use crate::parser::parser::parse;

pub fn find_cmago_dir() -> Option<PathBuf> {
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

pub fn find_cmago() -> Option<PathBuf> {
    // 从当前目录开始
    let mut dir = std::env::current_dir().ok()?;

    loop {
        let candidate = dir.join("Cmago.toml");

        if candidate.exists() && candidate.is_file() {
            return Some(dir.join("Cmago.toml")); // 找到就返回所在目录
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

pub fn ensure_file(path: &Path){
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let hello_world="\
#include <iostream>

int main(int argc, char *argv[]){
    std::cout << \"Hello, World!\" << std::endl;
    return 0;
}
";
    if !path.exists() {
        let mut file = File::create(path).expect(format!("Could not create {}", path.to_str().unwrap()).as_str());
        file.write_all(hello_world.as_bytes()).expect(format!("Could not write to {}", path.to_str().unwrap()).as_str());
    }
}

pub fn extract_repo_name(url: &str) -> Option<&str> {
    let trimmed = url.trim_end_matches('/')
        .trim_end_matches(".git");
    trimmed.rsplit('/').next()
}