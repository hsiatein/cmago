use super::{as_bin::AsBin};
use crate::cmake_config::dependencies::Dependencies;
use crate::cmake_config::has_dependencies::HasDependencies;

pub struct BinConfig{
    name:String,
    path:String,
    dependencies:Dependencies

}

impl BinConfig {
    pub fn new(name:&str,path:&str)->Self{
        BinConfig { name:name.to_string(), path:path.to_string(), dependencies: Dependencies::new() }
    }
    
}

impl AsBin for BinConfig {
    fn get_name(&self) -> &str {
        &self.name.as_str()
    }
    fn get_path(&self) -> &str {
        &self.path.as_str()
    }
}

impl HasDependencies for BinConfig{
    fn get_dependencies(&self) -> &Dependencies {
        &self.dependencies
    }
    
    fn get_mut_dependencies(&mut self) -> &mut Dependencies {
        &mut self.dependencies
    }
}