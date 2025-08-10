use std::io::Write;
use std::process::{Command, exit, Output, Stdio};

fn call(command: &str)->Output {
    let args: Vec<&str> = command.split_whitespace().collect();
    let mut command= Command::new(args[0].to_string());
    for arg in &args[1..] {
        command.arg(arg);
    }
    command.stdout(Stdio::inherit()).stderr(Stdio::inherit()).output().expect(format!("Failed to execute command: {command:?}").as_str())
}

pub fn download_dependency(url:&str,path:&str){
    let output = call(format!("git clone --recursive {url} {path}").as_str());
    println!("status: {}", output.status);
}

pub fn init_repository(path:&str){
    let output = call(format!("git init {path}").as_str());
    println!("status: {}", output.status);
}

pub fn add_dependency(url:&str,path:&str){
    let output = call(format!("git submodule add {url} {path}").as_str());
    println!("status: {}", output.status);
}

pub fn update_dependencies(){
    let output = call("git submodule update --init --recursive");
    println!("status: {}", output.status);
}

pub fn configuring(url:&str,path:&str){
    let output = call(format!("git clone --recursive {url} {path}").as_str());
    println!("status: {}", output.status);
}