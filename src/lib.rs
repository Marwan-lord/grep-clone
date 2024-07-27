use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name string"),
        };
        Ok(Config { query, file_name })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name);

    let contents = match contents {
        Ok(good) => good,
        Err(err) => return Err(err.into()),
    };

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three";

        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }
    #[test]
    fn test2() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive
Pick three
Trust the process";
        assert_eq!(vec!["Trust the process"], search(query, contents))
    }
}
