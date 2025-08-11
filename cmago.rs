use std::env;
use cmago::cmago::{init::init, new::new, update::update, add::add, build::build,run::run};

// use argparse::{ArgumentParser, StoreTrue, Store};

// fn main() {


//     let mut operation = String::new();
//     let mut arg = String::new();
//     let mut release = false;
//     {
//         let mut ap = ArgumentParser::new();
//         ap.set_description("Cmago - Cargo风味C++项目管理工具");
//         ap.refer(&mut operation)
//             .add_argument("operation", Store, r#"
//         init: 初始化一个空目录为Cmago项目
//         new: 新建一个目录, 名称为arg的值, 并初始化
//         update: 根据当前Cmago.toml配置文件, 更新包与CMakeLists
//         add: 为项目新增外部库, arg为该外部库git仓库
//         build: 按照CMakeLists进行编译, arg为空是编译所有目标, 否则编译名为arg的目标
//         run: 编译并运行名为arg的目标
//     "#);
//         ap.refer(&mut arg)
//             .add_argument("arg", Store, "operation的参数");
//         ap.refer(&mut release)
//             .add_option(&["--release"], StoreTrue, "以release模式编译");
//         ap.parse_args_or_exit();
//     }
//     // println!("operation: {}",operation);
//     // println!("arg: {}",arg);
    
//     if operation == "init" {
//         let current_dir = env::current_dir().unwrap();
//         init(current_dir.as_path());
//     }else if operation == "new" {
//         new(arg.as_str());
//     }else if operation == "update" {
//         update();
//     }else if operation == "add" {
//         add(arg.as_str());
//     }else if operation == "build" {
//         build(arg.as_str(),release);
//     }else if operation == "run" {
//         run(arg.as_str(),release);
//     }

// }

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Cmago")
        .version("1.0")
        .author("Author")
        .about("Cargo风味C++项目管理工具")
        .subcommand(
            Command::new("init")
                .about("初始化一个空目录为Cmago项目")
        )
        .subcommand(
            Command::new("new")
                .about("新建一个目录, 名称为arg的值, 并初始化")
                .arg(
                    Arg::new("arg")
                        .help("新建目录的名称")
                        .required(true)
                        .num_args(1)  // 参数需要一个值
                )
        )
        .subcommand(
            Command::new("update")
                .about("根据当前Cmago.toml配置文件, 更新包与CMakeLists")
        )
        .subcommand(
            Command::new("add")
                .about("为项目新增外部库, arg为该外部库git仓库")
                .arg(
                    Arg::new("arg")
                        .help("外部库的git仓库")
                        .required(true)
                        .num_args(1)  // 参数需要一个值
                )
        )
        .subcommand(
            Command::new("build")
                .about("按照CMakeLists进行编译")
                .arg(
                    Arg::new("arg")
                        .help("编译目标的名称，若为空则编译所有目标")
                        .num_args(0..=1)  // 参数是可选的
                        .default_value("")
                )
                .arg(
                    Arg::new("release")
                        .long("release")
                        .help("以release模式编译")
                        .num_args(0)
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("run")
                .about("编译并运行名为arg的目标")
                .arg(
                    Arg::new("arg")
                        .help("编译并运行的目标")
                        .required(true)
                        .num_args(1)  // 参数需要一个值
                )
                .arg(
                    Arg::new("release")
                        .long("release")
                        .help("以release模式编译")
                        .num_args(0)  // 参数不需要值
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .get_matches();
    let empty_string=String::new();
    match matches.subcommand() {
        Some(("init", _)) => {
            let current_dir = env::current_dir().unwrap();
            init(current_dir.as_path());
        }
        Some(("new", sub_matches)) => {
            let arg = sub_matches.get_one::<String>("arg").unwrap();
            new(arg);
        }
        Some(("update", _)) => {
            update();
        }
        Some(("add", sub_matches)) => {
            let arg = sub_matches.get_one::<String>("arg").unwrap();
            add(arg);
        }
        Some(("build", sub_matches)) => {
            let arg = sub_matches.get_one::<String>("arg").unwrap_or(&empty_string);
            let release = sub_matches.contains_id("release");
            build(arg, release);
        }
        Some(("run", sub_matches)) => {
            let arg = sub_matches.get_one::<String>("arg").unwrap();
            let release = sub_matches.contains_id("release");
            run(arg, release);
        }
        _ => {
            // Handle invalid or missing subcommand
            eprintln!("No valid operation provided");
        }
    }
}