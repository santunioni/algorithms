use crate::search::{search, search_case_insensitive};
use crate::search_input::SearchInput;
use std::{fs, process};

pub fn run(args: Vec<String>) {
    let search_input = SearchInput::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Can I read and print the contents iteratively in a stream?
    // Instead of reading all contents to array.
    let contents = fs::read_to_string(&search_input.file_path).unwrap_or_else(|err| {
        eprintln!("Problem opening file: {err}");
        process::exit(1)
    });

    let search_result_iter: Box<dyn Iterator<Item=&str>>;
    if search_input.ignore_case {
        let iter = search_case_insensitive(&search_input.query, &contents);
        search_result_iter = Box::new(iter);
    } else {
        let iter = search(&search_input.query, &contents);
        search_result_iter = Box::new(iter);
    };

    for line in search_result_iter {
        println!("{}", line)
    }
}
