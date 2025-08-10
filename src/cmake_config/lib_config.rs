use std::rc::Rc;
use crate::cmake_config::bin_config::BinConfig;
use super::as_lib::AsLib;
use crate::cmake_config::dependencies::Dependencies;
use super::has_dependencies::HasDependencies;

pub struct LibConfig{
    name:String,
    path:String,
    dependencies:Dependencies,

}

impl LibConfig {
    pub fn new(name:&str,path:&str)->LibConfig{
        LibConfig { name:name.to_string(), path:path.to_string(), dependencies: Dependencies::new() }
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
    fn add_dependency(&mut self, dep: &Rc<dyn AsLib>) {
        &self.dependencies.add_dependency(dep);
    }
}