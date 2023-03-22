use std::env;
use std::process;

use c_120_minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = c_120_minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

