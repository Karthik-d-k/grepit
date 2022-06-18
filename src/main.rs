use std::{env, process};

use grepit::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Usage: {} <query> <filename>", &args[0]);

        process::exit(1);
    });

    if let Err(e) = grepit::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);   
    }

}
