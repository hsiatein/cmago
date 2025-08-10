use crate::cmago::utils::find_cmago_dir;
use crate::call_tools::{cmake_build, cmake_build_target, cmake_configure};

pub fn build(name:&str,release:bool){
    let cmago_path = find_cmago_dir().unwrap();
    let cmago_path = cmago_path.to_str().unwrap();
    cmake_configure(cmago_path,release);
    if(name==""){
        cmake_build(cmago_path);
    }else { 
        cmake_build_target(cmago_path,name);
    }

}