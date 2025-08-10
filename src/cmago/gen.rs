use crate::cmake_config::cmake_config::CmakeConfig;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use crate::parser::parser::CmagoToml;
use std::path::Path;
use crate::parser::parser::parse;
use crate::cmake_generator::{to_main_cmakelists,to_sub_cmakelists};
use colored::Colorize;
use crate::cmake_config::as_lib::AsLib;
use crate::cmago::utils::get_cmake_config;

pub fn gen(path: &Path){
    let cmake_config = get_cmake_config(path);
    let cmake_lists = to_main_cmakelists(&cmake_config);
    cmake_lists.write_into_cmake_lists(path);
    for lib in &cmake_config.libraries{
        let target_dir = path.join(lib.get_name());
        if(!target_dir.exists()){
            fs::create_dir(&target_dir).expect("Could not create directory");
            fs::create_dir(target_dir.as_path().join("include")).expect("Could not create directory");
            fs::create_dir(target_dir.as_path().join("src")).expect("Could not create directory");
        }
        let cmake_lists = to_sub_cmakelists(&cmake_config,lib.get_name());
        cmake_lists.write_into_cmake_lists(target_dir.as_path());
    }
    
}