use std::env;
use std::fs::{self, File};
use std::io::Write;
use crate::parser::parser::CmagoToml;
use std::path::Path;
use colored::Colorize;
use crate::cmago::gen::gen;

pub fn init(){
    // 获取当前工作目录
    let current_dir = env::current_dir().unwrap();
    let name = current_dir.file_name().unwrap().to_str().unwrap();
    // 构造文件路径
    let file_path = current_dir.join("Cmago.toml");
    // 创建并写入 cmago.toml 文件
    let mut file = File::create(file_path).expect("Could not create Cmago.toml");
    let mut cmago_toml = CmagoToml::new();
    cmago_toml.package.project=name.to_string();
    let toml = cmago_toml.to_string();
    file.write_all(toml.as_bytes()).expect("Could not write Cmago.toml");
    println!("{}", format!("Created cmago.toml file in: {}", current_dir.display()).green());
    gen(current_dir.as_path());
}