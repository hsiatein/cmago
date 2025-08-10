use std::env;
use std::fs::{self, File};
use std::io::Write;
use crate::parser::parser::CmagoToml;
use std::path::Path;
use colored::Colorize;
use crate::call_tools::init_repository;
use crate::cmago::gen::gen;

pub fn init(current_dir: &Path){
    let mut entries = fs::read_dir(&current_dir).unwrap(); // 读取目录
    if !entries.next().is_none() {
        panic!("this directory is not empty");
    } 
    let name = current_dir.file_name().unwrap().to_str().unwrap();
    let file_path = current_dir.join("Cmago.toml");
    let mut file = File::create(file_path).expect("Could not create Cmago.toml");
    let mut cmago_toml = CmagoToml::new();
    cmago_toml.package.project=name.to_string();
    let toml = cmago_toml.to_string();
    file.write_all(toml.as_bytes()).expect("Could not write Cmago.toml");
    println!("{}", format!("Created cmago.toml file in: {}", current_dir.display()).green());
    gen(current_dir);
    init_repository(current_dir.to_str().unwrap());
    let mut file = File::create(current_dir.join(".gitignore")).expect("Could not create .gitignore");
    file.write_all("build/".as_bytes()).expect("Could not write to .gitignore");
}