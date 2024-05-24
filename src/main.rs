// use volpotorotat::vptr::convert::convert;
// use volpotorotat::vptr::tools::tools;
use volpotorotat::vptr::VERSION;
use volpotorotat::vptr;

mod cli;
mod repl;

use cli::cli::run_cli;
use repl::repl::run_repl;

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return;
    }

    if args[1] == "--repl" || args[1] == "repl" || args[1] == "-R" {
        //
        run_repl();
    }

    run_cli(&args);
}
