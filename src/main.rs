use std::{env, process};

use grepit::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Usage: <grepit> <query> <filename>");

        process::exit(1);
    });

    if let Err(e) = grepit::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);   
    }

}
