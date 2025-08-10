use std::collections::HashMap;
use serde::{Deserialize, Deserializer, Serialize,Serializer};
use crate::cmake_config::external_config::ExternalConfig;
use crate::parser::deps::AlterDep::Simple;

#[derive(Debug)]
pub struct Deps{
    pub deps: Vec<Dep>,
}

impl Deps{
    pub fn new() -> Self{
        Deps{deps:vec![Dep::new("gtest",Simple("https://github.com/google/googletest.git".to_string()))]}
    }
    pub fn to_external_configs(&self,external_path:&str)->Vec<ExternalConfig>{
        let mut external_configs = Vec::new();
        for dep in &self.deps{
            external_configs.push(dep.to_external_config(external_path));
        }
        external_configs
    }
    pub fn add_dep(&mut self, name: &str, url:&str){
        self.deps.push(Dep{name:name.to_string(),url:url.to_string()});
    }
}

impl Serialize for Deps {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = HashMap::new();
        for dep in &self.deps {
            let mut dep_map = HashMap::new();
            dep_map.insert("url", &dep.url);
            map.insert(&dep.name, dep_map);
        }
        serializer.collect_map(map)
    }
}

impl<'de> Deserialize<'de> for Deps {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut deps = Vec::new();
        let result: HashMap<String,AlterDep> = Deserialize::deserialize(deserializer)?;
        for (name,alter_dep) in result {
            deps.push(Dep::new(name.as_str(),alter_dep));
        }
        Ok(Deps{deps})
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dep {
    pub name: String,
    pub url: String,
}

impl Dep {
    fn new_simple(name:&str,url: &str)->Self{
        Dep{name:name.to_string(),url:url.to_string()}
    }
    fn new_detailed(name:&str,detail:TempDep)->Self{
        Dep{name:name.to_string(),url:detail.url}
    }
    pub fn new(name:&str,alter_dep: AlterDep)->Self{
        match alter_dep {
            AlterDep::Simple(url)=>{
                Dep::new_simple(name,url.as_str())
            }
            AlterDep::Detailed(detail)=>{
                Dep::new_detailed(name,detail)
            }
        }
    }
    
    pub fn to_external_config(&self,external_path:&str)->ExternalConfig{
        ExternalConfig::new(self.name.clone(),self.url.clone(),external_path.to_string())
    }
}


#[derive(Debug, Deserialize)]
pub struct TempDep {
    pub url: String,
}
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AlterDep {
    Simple(String),     // 处理字符串格式的 URL
    Detailed(TempDep), // 处理对象格式的 URL
}
