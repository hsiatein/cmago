use std::fs;
use std::path::Path;
use crate::cmake_generator::{to_main_cmakelists,to_sub_cmakelists};
use crate::cmake_config::as_lib::AsLib;
use crate::cmago::utils::{ensure_file, get_cmake_config};
use crate::cmake_config::as_bin::AsBin;

pub fn gen(path: &Path){
    let cmake_config = get_cmake_config(path);
    let cmake_lists = to_main_cmakelists(&cmake_config);
    cmake_lists.write_into_cmake_lists(path);
    if cmake_config.tests.path!=""{
        let tests_path=path.join(cmake_config.tests.path.as_str());
        if !tests_path.exists() {
            fs::create_dir(&tests_path).expect("Could not create directory");
        }
    }
    for bin in &cmake_config.binaries{
        ensure_file(path.join(bin.get_path()).as_path());
    }
    for lib in &cmake_config.libraries{
        let target_dir = path.join(lib.get_name());
        if !target_dir.exists() {
            fs::create_dir(&target_dir).expect("Could not create directory");
            fs::create_dir(target_dir.as_path().join("include")).expect("Could not create directory");
            fs::create_dir(target_dir.as_path().join("src")).expect("Could not create directory");
        }
        let cmake_lists = to_sub_cmakelists(&cmake_config,lib.get_name());
        cmake_lists.write_into_cmake_lists(target_dir.as_path());
    }

}