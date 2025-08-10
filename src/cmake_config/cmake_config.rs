use std::collections::HashSet;
use crate::cmake_config::as_bin::AsBin;
use crate::cmake_config::as_lib::AsLib;
use crate::cmake_config::{bin_config::BinConfig, external_config::ExternalConfig, lib_config::LibConfig, tests_config::TestsConfig};
use std::rc::Rc;
use crate::cmake_config::has_dependencies::HasDependencies;

pub struct CmakeConfig{
    pub project:String,
    pub version:String,
    pub cmake_minimum_required:String,
    pub cpp_standard:String,
    pub external_library_path:String,
    pub binaries:Vec<Rc<BinConfig>>,
    pub libraries:Vec<Rc<LibConfig>>,
    pub externals:Vec<Rc<ExternalConfig>>,
    pub tests:TestsConfig


}

impl CmakeConfig {
    pub fn new(project:String,version:String,cmake_minimum_required:String,cpp_standard:String,external_library_path:String)->Self{
        CmakeConfig { project, version, cmake_minimum_required, cpp_standard, external_library_path, binaries: Vec::new(), libraries: Vec::new(), externals: Vec::new(),tests: TestsConfig::new() }
    }

    pub fn add_bin(&mut self, bin:BinConfig){
        self.binaries.push(Rc::new(bin));

    }

    pub fn add_lib(&mut self, lib:LibConfig){
        self.libraries.push(Rc::new(lib));
    }

    pub fn add_dep(&mut self, external:ExternalConfig){
        self.externals.push(Rc::new(external));
    }

    pub fn set_deps(&mut self, externals:Vec<ExternalConfig>){
        for external in externals {
            self.add_dep(external);
        }
    }

    pub fn has_lib(&mut self, name:&str)->bool{
        for dep in &self.externals{
            if dep.get_name() == name{
                return true;
            }
        }
        for lib in &self.libraries{
            if lib.get_name() == name{
                return true;
            }
        }
        false
    }

    pub fn get_lib(&self, name:&str) -> Rc<dyn AsLib>{
        let find = self.externals.iter().find(|dep| dep.get_name() == name);
        if let Some(dep) = find{
            return dep.clone();
        }
        let find = self.libraries.iter().find(|lib| lib.get_name() == name);
        find.expect(&format!("lib {} not found", name)).clone()

    }

    pub fn needed_deps(&self)->Vec<Rc<ExternalConfig>>{
        let mut result=Vec::new();
        if(self.tests.path!=""){
            if let Some(ext) = self.externals.iter().find(|ext| ext.get_name()=="gtest"){
                result.push(ext.clone());
            }
            
        }
        for bin in &self.binaries{
            for dep in &bin.get_dependencies().dependencies{
                if let Some(ext) = self.externals.iter().find(|ext| ext.get_name()==dep.get_name()){
                    if result.iter().find(|res| res.get_name()==ext.get_name()).is_none(){
                        result.push(ext.clone());
                    }
                }
            }
        }
        for lib in &self.libraries{
            for dep in &lib.get_dependencies().dependencies{
                if let Some(ext) = self.externals.iter().find(|ext| ext.get_name()==dep.get_name()){
                    if result.iter().find(|res| res.get_name()==ext.get_name()).is_none(){
                        result.push(ext.clone());
                    }
                }
            }
        }
        result
    }
}