use std::fs::File;
use std::io::Write;
use colored::Colorize;
use crate::cmago::update::update;
use crate::cmago::utils::{find_cmago, get_cmake_config};
use crate::parser::parser::CmagoToml;

pub fn add(url:&str){
    let cmago_path = find_cmago().unwrap();
    let mut cmago_toml = CmagoToml::from_path(cmago_path.to_str().unwrap());
    cmago_toml.deps.add_dep(extract_repo_name(url).unwrap(),url);
    let toml = cmago_toml.to_string();
    let mut file = File::create(cmago_path.as_path()).expect("Could not create Cmago.toml");
    file.write_all(toml.as_bytes()).expect("Could not write Cmago.toml");
    println!("{}", format!("Write cmago.toml : {}", cmago_path.display()).green());
    update()
}

fn extract_repo_name(url: &str) -> Option<&str> {
    let trimmed = url.trim_end_matches('/')
        .trim_end_matches(".git");
    trimmed.rsplit('/').next()
}