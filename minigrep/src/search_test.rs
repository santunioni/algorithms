use crate::search::{search, search_case_insensitive};

#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    let result: Vec<&str> = search(query, contents).collect();
    assert_eq!(vec!["safe, fast, productive."], result);
}

#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    let result: Vec<&str> = search_case_insensitive(query, contents).collect();
    assert_eq!(
        vec!["Rust:", "Trust me."],
        result
    );
}
