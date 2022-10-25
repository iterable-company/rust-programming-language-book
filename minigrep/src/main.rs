use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });
    if let Err(e) = run(config) {
        println!("{}", e);
        process::exit(1)
    }
}
