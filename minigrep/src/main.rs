use minigrep::run;
use std::env;

mod search_input;

fn main() {
    let args = env::args().collect();
    run::run(args)
}
