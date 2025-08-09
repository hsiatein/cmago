use super::as_lib::AsLib;
use std::rc::Rc;

pub struct LibConfig{
    name:String,
    path:String,
    dependencies:Vec<Rc<dyn AsLib>>

}

impl LibConfig {
    fn new(name:String,path:String)->LibConfig{
        LibConfig { name: name, path: path, dependencies: Vec::new() }
    }
}

impl AsLib for LibConfig {
    fn get_path<'a>(&'a self)->&'a String {
        &self.path
    }
}