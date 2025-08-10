use std::fmt::format;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use colored::Colorize;

pub struct CMakeLists{
    pub context: String,
}

impl CMakeLists{
    pub fn new() -> CMakeLists{
        CMakeLists{context: String::new()}
    }
    
    pub fn write_into_cmake_lists(&self, path:&Path){
        let mut file = File::create(path.join("CMakeLists.txt")).expect("Could not create CMakeLists.txt");
        file.write_all(self.context.as_bytes()).expect("Could not write CMakeLists.txt");
        println!("{}", format!("Created CMakeLists.txt file in: {}", path.display()).green());
    }
    
    fn write_line(&mut self,line:String){
        self.context.push_str(line.as_str());
        self.context.push('\n');
    }
    fn write(&mut self,line:String){
        self.context.push_str(line.as_str());
    }
    pub fn tests(&mut self,gtest_path:&str,tests_path:&str){
        self.write_line("#tests".to_string());
        let path = Path::new(gtest_path).join("googletest").join("src").join("gtest_main.cc");
        self.set("TEST_MAIN_FUNC",path.to_str().unwrap());
        let path = Path::new(tests_path).join("*.cpp");
        self.file("TEST_FILES",path.to_str().unwrap());
        self.write_line("foreach(file ${TEST_FILES})".to_string());
        self.write_line(r"    get_filename_component(name ${file} NAME_WE)".to_string());
        self.write_line(r"    add_executable(${name} ${file} ${TEST_MAIN_FUNC})".to_string());
        self.write("    ".to_string());
        let deps=vec!["gtest"];
        self.target_link_libraries("${name}",deps.clone());
        self.write_line("endforeach()".to_string());
        self.write_line("add_executable(test_all ${TEST_MAIN_FUNC} ${TEST_FILES})".to_string());
        self.target_link_libraries("test_all",deps);
    }

    pub fn set(&mut self,name:&str, value:&str) {
        self.write_line(format!("set({name} {value})"));
    }
    pub fn cmake_minimum_required(&mut self,version:&str){
        self.write_line(format!("cmake_minimum_required(VERSION {version})"));
    }

    pub fn project(&mut self,name:&str){
        self.write_line(format!("project({name} CXX)"));
    }


    pub fn add_subdirectory(&mut self,dir:&str){
        self.write_line(format!("add_subdirectory({dir})"));
    }

    /// access=PUBLIC,PRIVATE
    pub fn target_compile_features(&mut self,name:&str, access:&str, value:&str){
        self.write_line(format!("target_compile_features({name} {access} {value})"));
    }

    pub fn set_cxx_standard(&mut self,name:&str, value:&str){
        self.target_compile_features(name,"PUBLIC",format!("cxx_std_{value}").as_str())
    }

    pub fn add_executable(&mut self,name:&str, path:&str){
        self.write_line(format!("add_executable({name} {path})"));
    }

    /// status=STATIC,SHARED
    pub fn add_library(&mut self,name:&str, status:&str, path:&str){
        self.write_line(format!("add_library({name} {status} {path})"));
    }

    pub fn target_link_libraries(&mut self,name:&str,deps:Vec<&str>){
        let mut start=format!("target_link_libraries({name}");
        for dep in deps {
            start.push(' ');
            start.push_str(dep);
        }
        start.push(')');
        self.write_line(start);
    }

    pub fn file(&mut self,name:&str,pattern:&str){
        self.write_line(format!("file(GLOB {name} {pattern})"));
    }

    /// access=PUBLIC,PRIVATE,INTERFACE
    pub fn target_include_directories(&mut self,name:&str, access:&str, dir:&str){
        self.write_line(format!("target_include_directories({name} {access} {dir})"));
    }
}
