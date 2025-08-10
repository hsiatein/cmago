use crate::cmago::utils::{find_cmago_dir, get_cmake_config};
use crate::call_tools::download_dependency;
use crate::cmago::gen::gen;
use crate::cmake_config::as_lib::AsLib;

pub fn update(){
    let cmago_path = find_cmago_dir().unwrap();
    let cmake_config = get_cmake_config(cmago_path.as_path());
    for dep in cmake_config.externals{
        download_dependency(dep.get_url(),dep.get_path());
    }
    gen(cmago_path.as_path());
}