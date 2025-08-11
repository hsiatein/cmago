use super::as_lib::AsLib;
use crate::cmake_config::dependencies::Dependencies;
use super::has_dependencies::HasDependencies;

pub struct LibConfig{
    name:String,
    path:String,
    lib_type:String,
    position_independent:bool,
    dependencies:Dependencies,

}

impl LibConfig {
    pub fn new(name:&str,path:&str,lib_type:&str)->LibConfig{
        LibConfig { name:name.to_string(), path:path.to_string(),lib_type:lib_type.to_string(),position_independent:false, dependencies: Dependencies::new() }
    }
    
    pub fn get_lib_type(&self)->&str{
        &self.lib_type.as_str()
    }

    pub fn get_pic(&self)->bool{
        self.position_independent
    }

    pub fn set_pic(&mut self, pic:bool){
        self.position_independent=pic;
    }
}

impl AsLib for LibConfig {
    fn get_path(&self) -> &str {
        &self.path.as_str()
    }
    fn get_name(&self) -> &str {
        &self.name.as_str()
    }
}

impl HasDependencies for LibConfig{
    fn get_dependencies(&self) -> &Dependencies {
        &self.dependencies
    }
    fn get_mut_dependencies(&mut self) -> &mut Dependencies {
        &mut self.dependencies
    }
}