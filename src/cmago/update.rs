use crate::cmago::utils::{find_cmago_dir, get_cmake_config};
use crate::call_tools::{add_dependency, update_dependencies};
use crate::cmago::gen::gen;
use crate::cmake_config::as_lib::AsLib;
use std::fs;

pub fn update(){
    let cmago_path = find_cmago_dir().unwrap();
    let cmake_config = get_cmake_config(cmago_path.as_path());
    let gitmodules_path = cmago_path.join(".gitmodules");
    let mut content = String::new();
    if gitmodules_path.exists(){
        content = fs::read_to_string(&gitmodules_path).unwrap();
    }
    for dep in cmake_config.externals{
        if content.contains(dep.get_url()){
            continue;
        }
        add_dependency(dep.get_url(),dep.get_path());
    }
    update_dependencies();
    gen(cmago_path.as_path());
}