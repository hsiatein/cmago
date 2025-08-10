use std::env;
use cmago::cmago::{init::init, new::new, update::update, add::add, build::build};

use argparse::{ArgumentParser, StoreTrue, Store};
use cmago::cmago::run::run;

fn main() {


    let mut operation = String::new();
    let mut arg = String::new();
    let mut release = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("cmago - A tool for managing cmake configurations");
        ap.refer(&mut operation)
            .add_argument("operation", Store, "Operations: init, new, update...");
        ap.refer(&mut arg)
            .add_argument("arg", Store, "argument for the operation");
        ap.refer(&mut release)
            .add_option(&["--release"], StoreTrue, "release in building");
        ap.parse_args_or_exit();
    }
    // println!("operation: {}",operation);
    // println!("arg: {}",arg);
    
    if operation == "init" {
        let current_dir = env::current_dir().unwrap();
        init(current_dir.as_path());
    }else if operation == "new" {
        new(arg.as_str());
    }else if operation == "update" {
        update();
    }else if operation == "add" {
        add(arg.as_str());
    }else if operation == "build" {
        build(arg.as_str(),release);
    }else if operation == "run" {
        run(arg.as_str(),release);
    }

}