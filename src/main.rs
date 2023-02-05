extern crate minihead;

use std::env;
use std::process;

use minihead::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    if let Err(e) = minihead::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
