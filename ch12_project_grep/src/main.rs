mod lib;

use std::{env, fs, process};
use std::error::Error;
use crate::lib::{run, Config};

fn main() {
    /// 接收传递的参数
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    println!("{:?}", args);

    // let config = parse_config(&args);  // old
    if let Ok(config) = Config::new(&args) {
        println!("searching for {}", config.query);
        println!("checking file {}", config.file_path);
        // run(config);

        if let Err(e) = lib::run(config) {
            println!("Application error: {}", e);
            process::exit(1);
        }
    }
    
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem building the configuration: {}", err);
        process::exit(1);
    });
}


fn parse_config(args: &Vec<String>) -> Config {
    let query = &args[1];
    let file_path = args[2].clone();
    let cmd = &args[0];
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    Config {
        query: query.to_string(),
        file_path,
        cmd: cmd.clone(),
        ignore_case,
    }
}

struct BetterConfig<'a> {
    query: &'a String,
    file_path: &'a String,
}

fn parse_better(args: &Vec<String>) -> BetterConfig {
    let query = &args[2];
    let file_path = &args[3];
    BetterConfig {
        query,
        file_path,
    }
}