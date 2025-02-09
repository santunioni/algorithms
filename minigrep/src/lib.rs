mod minigrep;
mod search;
mod search_input;

pub use minigrep::run_with_args;
pub use minigrep::run_with_command;

#[cfg(test)]
mod search_test;
