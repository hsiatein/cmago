use crate::cmake_config::bin_config::BinConfig;
use crate::{cmake_config::cmake_config::CmakeConfig, cmake_generator::command::CMakeLists};
use crate::cmake_config::as_lib::AsLib;
use std::path::Path;
use crate::cmake_config::as_bin::AsBin;
use crate::cmake_config::has_dependencies::HasDependencies;

pub fn base_cmake(cmake_lists:&mut CMakeLists,cmake_config:&CmakeConfig){
    cmake_lists.cmake_minimum_required(cmake_config.cmake_minimum_required.as_str());
    cmake_lists.project(cmake_config.project.as_str());
    cmake_lists.set("PROJECT_VERSION", cmake_config.version.as_str());
    cmake_lists.write_line("if (MSVC)");
    cmake_lists.write_line("    add_compile_options(/utf-8)");
    cmake_lists.write_line("endif()");
    cmake_lists.write_line("");
}

pub fn tests_cmake(cmake_lists:&mut CMakeLists,cmake_config:&CmakeConfig){
    let gtest_path=cmake_config.get_lib("gtest").get_path().to_string();
    let tests_path=cmake_config.tests.path.as_str().to_string();
    cmake_lists.write_line("#tests");
    let path = Path::new(gtest_path.as_str()).join("googletest").join("src").join("gtest_main.cc");
    cmake_lists.set("TEST_MAIN_FUNC",path.to_str().unwrap());
    
    let path = Path::new(tests_path.as_str()).join("*.cpp");
    cmake_lists.file("TEST_FILES",path.to_str().unwrap());
    cmake_lists.write_line("foreach(file ${TEST_FILES})");
    cmake_lists.write_line(r"    get_filename_component(name ${file} NAME_WE)");
    cmake_lists.write_line(r"    add_executable(${name} ${file} ${TEST_MAIN_FUNC})");
    cmake_lists.write("    ");
    let mut deps=vec!["gtest"];
    for lib in &cmake_config.libraries{
        deps.push(lib.get_name());
    }
    cmake_lists.target_link_libraries("${name}",deps.clone());
    cmake_lists.write_line("endforeach()");
    cmake_lists.write_line("add_executable(test_all ${TEST_MAIN_FUNC} ${TEST_FILES})");
    cmake_lists.target_link_libraries("test_all",deps);
}

pub fn exec_cmake(cmake_lists:&mut CMakeLists,cmake_config:&CmakeConfig,bin:&BinConfig){
    cmake_lists.add_executable(bin.get_name(), bin.get_path());
    cmake_lists.set_cxx_standard(bin.get_name(), cmake_config.cpp_standard.as_str());
    if bin.get_str_deps().len() > 0{
        cmake_lists.target_link_libraries(bin.get_name(),bin.get_str_deps());
    }
    cmake_lists.write_line("");
}