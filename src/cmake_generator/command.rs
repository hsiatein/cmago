use std::fs::File;
use std::io::Write;
use std::path::{Path};
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
    
    pub fn write_line(&mut self,line:&str){
        self.context.push_str(line);
        self.context.push('\n');
    }
    pub fn write(&mut self,line:&str){
        self.context.push_str(line);
    }

    pub fn set(&mut self,name:&str, value:&str) {
        self.write_line(format!("set({name} {value})").as_str());
    }
    pub fn cmake_minimum_required(&mut self,version:&str){
        self.write_line(format!("cmake_minimum_required(VERSION {version})").as_str());
    }

    pub fn project(&mut self,name:&str){
        self.write_line(format!("project({name} CXX)").as_str());
    }


    pub fn add_subdirectory(&mut self,dir:&str){
        self.write_line(format!("add_subdirectory({dir})").as_str());
    }

    /// access=PUBLIC,PRIVATE
    pub fn target_compile_features(&mut self,name:&str, access:&str, value:&str){
        self.write_line(format!("target_compile_features({name} {access} {value})").as_str());
    }

    pub fn set_cxx_standard(&mut self,name:&str, value:&str){
        self.target_compile_features(name,"PUBLIC",format!("cxx_std_{value}").as_str())
    }

    pub fn add_executable(&mut self,name:&str, path:&str){
        self.write_line(format!("add_executable({name} {path})").as_str());
    }

    /// status=STATIC,SHARED
    pub fn add_library(&mut self,name:&str, status:&str, path:&str){
        self.write_line(format!("add_library({name} {status} {path})").as_str());
    }

    pub fn set_position_independent(&mut self,name:&str){
        self.write_line(format!("set_target_properties({name} PROPERTIES POSITION_INDEPENDENT_CODE ON)").as_str());
    }
    

    pub fn target_link_libraries(&mut self,name:&str,deps:Vec<&str>){
        let mut start=format!("target_link_libraries({name}");
        for dep in deps {
            start.push(' ');
            start.push_str(dep);
        }
        start.push(')');
        self.write_line(start.as_str());
    }

    pub fn file(&mut self,name:&str,pattern:&str){
        self.write_line(format!("file(GLOB {name} {pattern})").as_str());
    }

    /// access=PUBLIC,PRIVATE,INTERFACE
    pub fn target_include_directories(&mut self,name:&str, access:&str, dir:&str){
        self.write_line(format!("target_include_directories({name} {access} {dir})").as_str());
    }
}
