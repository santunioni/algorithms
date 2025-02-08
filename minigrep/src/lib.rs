use std::error::Error;
use std::{env, fs};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // Can I read and print the contents iteratively in a stream?
    let contents = fs::read_to_string(&config.file_path)?;

    let search_result_iter: Box<dyn Iterator<Item=&str>>;
    if config.ignore_case {
        let iter = search_case_insensitive(&config.query, &contents);
        search_result_iter = Box::new(iter);
    } else {
        let iter = search(&config.query, &contents);
        search_result_iter = Box::new(iter);
    };

    for line in search_result_iter {
        println!("{}", line)
    }
    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> impl Iterator<Item=&'a str> + 'a {
    contents
        .lines()
        .filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> impl Iterator<Item=&'a str> + 'a {
    let lower_cased_query = query.to_lowercase();
    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&lower_cased_query))
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() <= 1 {
            return Err("You didn't provide search_query and file_path arguments");
        } else if args.len() == 2 {
            return Err("Missing file_path argument");
        }
        let query = args.remove(1);
        let file_path = args.remove(1);
        let ignore_case = match env::var("IGNORE_CASE") {
            Ok(value) => value.to_lowercase() != "false" && value != "0",
            Err(_) => false,
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod lib_test;