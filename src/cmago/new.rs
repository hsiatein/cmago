use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use colored::Colorize;
use crate::call_tools::init_repository;
use crate::cmago::gen::gen;
use crate::cmago::init::init;
use crate::parser::parser::CmagoToml;

pub fn new(name: &str){
    let current_dir = env::current_dir().unwrap();
    let target_dir = current_dir.join(name);

    if target_dir.exists() {
        panic!("Directory already exists: {}", target_dir.display());
    }
    fs::create_dir(&target_dir).expect("Could not create directory");
    println!("{}", format!("Created {} dir in: {}",target_dir.display(), current_dir.display()).green());
    init(target_dir.as_path());
}