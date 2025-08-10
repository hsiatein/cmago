use std::io::Write;
use std::path::Path;
use std::process::{Command, exit, Output, Stdio};

fn call(command: &str)->Output {
    let args: Vec<&str> = command.split_whitespace().collect();
    let mut command= Command::new(args[0].to_string());
    for arg in &args[1..] {
        command.arg(arg);
    }
    command.stdout(Stdio::inherit()).stderr(Stdio::inherit()).output().expect(format!("Failed to execute command: {command:?}").as_str())
}

// pub fn download_dependency(url:&str,path:&str){
//     let output = call(format!("git clone --recursive {url} {path}").as_str());
//     println!("status: {}", output.status);
// }

pub fn download_dependency(url:&str,path:&str){
    add_dependency(url,path);
    update_dependencies();
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


pub fn cmake_configure(cmake_lists_path:&str,release:bool){
    let build_dir= Path::new(cmake_lists_path).join("build");
    if(release){
        println!("release mode");
        let output = call(format!("cmake -S {} -B {} -G Ninja -DCMAKE_BUILD_TYPE=Release",cmake_lists_path,build_dir.to_str().unwrap()).as_str());
        println!("status: {}", output.status);
    }else {
        println!("debug mode");
        let output = call(format!("cmake -S {} -B {} -G Ninja",cmake_lists_path,build_dir.to_str().unwrap()).as_str());
        println!("status: {}", output.status);
    }
    
}

pub fn cmake_build(cmake_lists_path:&str){
    let build_dir= Path::new(cmake_lists_path).join("build");
    let output = call(format!("cmake --build {}",build_dir.to_str().unwrap()).as_str());
    println!("status: {}", output.status);
    
}

pub fn cmake_build_target(cmake_lists_path:&str,target:&str){
    let build_dir= Path::new(cmake_lists_path).join("build");
    let output = call(format!("cmake --build {} --target {}",build_dir.to_str().unwrap(),target).as_str());
    println!("status: {}", output.status);

}