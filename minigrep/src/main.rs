use minigrep_santunioni::run_with_args;
use std::env;

fn main() {
    let args = env::args().collect();
    run_with_args(args, true)
}
