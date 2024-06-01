// use volpotorotat::vptr::convert::convert;
// use volpotorotat::vptr::tools::tools;
use volpotorotat::vptr;
use volpotorotat::vptr::VERSION;

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
    } else {
        let mut cli_args = args.clone();
        cli_args.remove(0);
        run_cli(&cli_args);
    }
}
