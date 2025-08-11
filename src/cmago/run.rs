use std::fs::canonicalize;
use crate::cmago::build::build;
use crate::call_tools::run_executable;
use crate::cmago::utils::find_cmago_dir;

pub fn run(name:&str, release:bool){
    build(name,release);
    let mut exec_path = canonicalize(find_cmago_dir().unwrap().join("build")).expect("Can't find cmago dir");
    if cfg!(target_os = "windows") {
        exec_path = exec_path.join(format!("{name}.exe").as_str());
    } else if cfg!(target_os = "linux") {
        exec_path = exec_path.join(name);
    } else if cfg!(target_os = "macos") {
        exec_path = exec_path.join(name);
    } else{
        panic!("unsupported OS");
    }
    run_executable(exec_path.to_str().unwrap());
}