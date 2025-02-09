mod run;
mod search;
mod search_input;

pub use run::run_with_args;
pub use run::run_with_command;

#[cfg(test)]
mod search_test;
