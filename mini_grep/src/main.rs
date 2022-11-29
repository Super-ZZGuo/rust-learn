use std::{
    env,
    process,
};
use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Parsing arguments error: {}", err);
        process::exit(1);
    });

    // println!("{:?}", config);

    println!("args query: {}", config.query);
    println!("args file_name: {}", config.file_name);

    // let content =  fs::read_to_string(config.file_name).expect("Read the file failed");

    // println!("content: {}", content);

    // run(config);

    if let Err(err) = mini_grep::run(config) {
        eprintln!("run error: {}", err);

        process::exit(1);
    }
}