use super::{as_bin::AsBin, as_lib::AsLib};
use std::rc::Rc;

pub struct BinConfig{
    name:String,
    path:String,
    dependencies:Vec<Rc<dyn AsLib>>

}

impl BinConfig {
    fn new(name:String,path:String)->BinConfig{
        BinConfig { name: name, path: path, dependencies: Vec::new() }
    }
    fn add_dependency(&mut self,dependency:&Rc<dyn AsLib>){
        self.dependencies.push(Rc::clone(dependency));
    }
}

impl AsBin for BinConfig {
    fn get_path<'a>(&'a self)->&'a String {
        &self.path
    }
}