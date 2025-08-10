use cmago::cmago::{init::init,new::new,update::update,add::add};

use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {


    // 定义第一个命令行参数：操作类型（例如 "operate"）
    let mut operation = String::new();
    let mut arg = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("cmago - A tool for managing cmake configurations");
        ap.refer(&mut operation)
            .add_argument("operation", Store, "Operations: init, new, uodate...");
        ap.refer(&mut arg)
            .add_argument("arg1", Store, "First argument for the operation");
        ap.parse_args_or_exit();
    }
    if operation == "init" {
        init();
    }else if operation == "new" {
        new(arg.as_str());
    }else if operation == "update" {
        update();
    }else if operation == "add" {
        add(arg.as_str());
    }

}