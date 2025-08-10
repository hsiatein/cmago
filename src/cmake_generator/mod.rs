pub mod command;

use crate::cmake_config::as_bin::AsBin;
use crate::cmake_config::as_lib::AsLib;
use crate::cmake_config::cmake_config::CmakeConfig;
use crate::cmake_config::has_dependencies::HasDependencies;
use crate::cmake_config::lib_config::LibConfig;
use crate::cmake_generator::command::{CMakeLists};

pub fn to_main_cmakelists(cmake_config: &CmakeConfig) ->CMakeLists{
    let mut cmake_lists=CMakeLists::new();
    cmake_lists.cmake_minimum_required(cmake_config.cmake_minimum_required.as_str());
    cmake_lists.project(cmake_config.project.as_str());
    cmake_lists.set("PROJECT_VERSION", cmake_config.version.as_str());
    for dep in &cmake_config.externals{
        cmake_lists.add_subdirectory(dep.get_path());
    }
    for lib in &cmake_config.libraries{
        cmake_lists.add_subdirectory(lib.get_path());
    }
    for bin in &cmake_config.binaries{
        cmake_lists.add_executable(bin.get_name(), bin.get_path());
        cmake_lists.set_cxx_standard(bin.get_name(), cmake_config.cpp_standard.as_str());
        cmake_lists.target_link_libraries(bin.get_name(),bin.get_str_deps())
    }
    if(cmake_config.tests.path!=""){
        cmake_lists.tests(cmake_config.get_lib("gtest").get_path(),cmake_config.tests.path.as_str());
    }
    cmake_lists
}

pub fn to_sub_cmakelists(cmake_config: &CmakeConfig,lib_name: &str) ->CMakeLists{
    let mut cmake_lists=CMakeLists::new();
    cmake_lists.cmake_minimum_required(cmake_config.cmake_minimum_required.as_str());
    cmake_lists.project(lib_name);
    cmake_lists.file("SRCS","./src/*.cpp");
    cmake_lists.add_library(lib_name,"PUBLIC",r"${SRCS}");
    cmake_lists.set_cxx_standard(lib_name, cmake_config.cpp_standard.as_str());
    cmake_lists.target_include_directories(lib_name,"PUBLIC","./include");
    let lib = cmake_config.libraries.iter().find(|lib| lib.get_name()==lib_name).unwrap();
    cmake_lists.target_link_libraries(lib_name,lib.get_str_deps());
    cmake_lists
}

