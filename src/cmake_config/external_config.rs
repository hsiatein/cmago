use std::path::Path;
use super::as_lib::AsLib;
use crate::cmago::utils::extract_repo_name;


pub struct ExternalConfig{
    name:String,
    url:String,
    path:String,
}

impl ExternalConfig {
    pub fn new(name:String,url:String,external_library_path:String)->Self{
        let dir = extract_repo_name(url.as_str()).unwrap();
        let path=Path::new(external_library_path.as_str()).join(dir).to_str().unwrap().to_string();
        ExternalConfig { name, url, path }
    }
    
    pub fn get_url(&self)->&str{
        self.url.as_str()
    }
}

impl AsLib for ExternalConfig {
    fn get_path(&self) -> &str {
        &self.path.as_str()
    }
    fn get_name(&self) -> &str {
        &self.name.as_str()
    }
}