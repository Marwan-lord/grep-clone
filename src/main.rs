use mini_grep::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing argumnets: {}", err);
        process::exit(1);
    });

    if let Err(e) = mini_grep::run(config) {
        println!("Application Error: {}", e);
        process::exit(1)
    }
}
