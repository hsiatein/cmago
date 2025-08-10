use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use colored::Colorize;
use crate::cmago::gen::gen;
use crate::parser::parser::CmagoToml;

pub fn new(name: &str){
    let current_dir = env::current_dir().unwrap();
    let target_dir = current_dir.join(name);

    if target_dir.exists() {
        panic!("Directory already exists: {}", target_dir.display());
    }
    fs::create_dir(&target_dir).expect("Could not create directory");
    println!("{}", format!("Created {} dir in: {}",target_dir.display(), current_dir.display()).green());
    let file_path = target_dir.join("Cmago.toml");
    let mut file = File::create(file_path).expect("Could not create Cmago.toml");
    let mut cmago_toml = CmagoToml::new();
    cmago_toml.package.project=name.to_string();
    let toml = cmago_toml.to_string();
    file.write_all(toml.as_bytes()).expect("Could not write Cmago.toml");
    println!("{}", format!("Created cmago.toml file in: {}", target_dir.display()).green());
    gen(target_dir.as_path());
}