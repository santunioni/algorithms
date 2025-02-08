use std::error::Error;
use std::{env, fs};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    // Can I read and print the contents iteratively in a stream?
    let contents = fs::read_to_string(&config.file_path)?;

    // Can I return an iterator of &str instead of a Vec<&str>?
    // I am creating an unnecessary array here.
    let ret = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in ret {
        println!("{}", line)
    }
    Ok(())
}

pub fn search<'b>(query: &str, contents: &'b str) -> Vec<&'b str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'b>(query: &str, contents: &'b str) -> Vec<&'b str> {
    let lower_cased_query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&lower_cased_query))
        .collect()
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