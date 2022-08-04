use std::error::Error;
use rust_project_08::utils;
use rust_project_08::utils::run;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    if !utils::parse_args(&args) {
        println!("Usage: cargo run -- transactions.csv > accounts.csv");
        std::process::exit(1)
    };

    match run(args[1].to_string()) {
        Ok(_) => {}
        Err(e) => { eprintln!("{}", e) }
    }
}

