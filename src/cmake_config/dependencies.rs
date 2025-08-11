use std::rc::Rc;
use crate::cmake_config::as_lib::AsLib;

pub struct Dependencies{
    pub dependencies:Vec<Rc<dyn AsLib>>
}

impl Dependencies {
    pub fn new()->Self{
        Dependencies{dependencies:Vec::new()}
    }
}

impl Dependencies {
    pub fn add_dependency(&mut self,dependency:&Rc<dyn AsLib>){
        self.dependencies.push(Rc::clone(dependency));
    }
}