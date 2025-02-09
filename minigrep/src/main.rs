use minigrep::run;
use std::env;

fn main() {
    let args = env::args().collect();
    run::run(args)
}
