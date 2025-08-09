use super::as_lib::AsLib;



pub struct ExternalConfig{
    name:String,
    url:String,
    path:String,
}

impl ExternalConfig {
    fn new(name:String,url:String,external_library_path:String)->ExternalConfig{
        let path=external_library_path+&name;
        ExternalConfig { name: name, url: url, path: path }
    }
}

impl AsLib for ExternalConfig {
    fn get_path<'a>(&'a self)->&'a String {
        &self.path
    }
}