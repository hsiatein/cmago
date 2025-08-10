use std::fs::File;
use std::io::Write;
use colored::Colorize;
use crate::cmago::update::update;
use crate::cmago::utils::{find_cmago, extract_repo_name};
use crate::parser::parser::CmagoToml;
use crate::famous_libraries::find_external;
use url::Url;

pub fn add(url:&str){
    let mut git_url=url.to_string();
    let mut name=None;;
    if Url::parse(url).is_ok(){
        if !url.ends_with(".git"){
            git_url = url.to_string()+".git";
        }
    }else if let Some(external) = find_external(url){
        git_url=external.to_string();
        name=Some(url);
    }
    if name.is_none(){
        name=Some(extract_repo_name(git_url.as_str()).unwrap());
    }
    let cmago_path = find_cmago().unwrap();
    let mut cmago_toml = CmagoToml::from_path(cmago_path.to_str().unwrap());
    cmago_toml.deps.add_dep(name.unwrap(),git_url.as_str());
    let toml = cmago_toml.to_string();
    let mut file = File::create(cmago_path.as_path()).expect("Could not create Cmago.toml");
    file.write_all(toml.as_bytes()).expect("Could not write Cmago.toml");
    println!("{}", format!("Write cmago.toml : {}", cmago_path.display()).green());
    update()
}
