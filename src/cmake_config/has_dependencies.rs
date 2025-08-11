use std::rc::Rc;
use crate::cmake_config::as_lib::AsLib;
use super::dependencies::Dependencies;

pub trait HasDependencies {
    fn get_dependencies(&self) -> &Dependencies;
    fn get_mut_dependencies(&mut self) -> &mut Dependencies;
    fn get_str_deps(&self) -> Vec<&str> {
        let mut deps=Vec::new();
        for dep in &self.get_dependencies().dependencies {
            deps.push(dep.get_name());
        }
        deps
    }
    fn add_dependency(&mut self, dep:&Rc<dyn AsLib>){
        self.get_mut_dependencies().add_dependency(dep);
    }
}