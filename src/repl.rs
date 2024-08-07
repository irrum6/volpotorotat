pub mod repl {
    use std::io::stdin;
    //crate
    use crate::vptr;
    use crate::VERSION;

    pub fn run_repl() {
        let mut exit = false;
        let mut line = String::new();

        loop {
            if exit {
                break;
            }

            stdin().read_line(&mut line).unwrap();
            let basic_arg = line.trim();
            if "exit" == basic_arg {
                exit = true;
                line.truncate(0);
                continue;
            }

            if "version" == basic_arg || "-V" == basic_arg {
                println!("Volpwtorq v{}", VERSION);
                line.truncate(0);
                continue;
            }

            let args: Vec<String> = basic_arg.split(" ").map(|e| String::from(e)).collect();

            if args.len() < 2 {
                println!("pass more parameters");
                continue;
            }
            if args[0].trim() == "vol" {
                vptr::print_vol(&args);
                line.truncate(0);
                continue;
            }

            if args[0] == "tolbf" || args[1] == "TL" {
                vptr::print_lbf(&args);
                line.truncate(0);
                continue;
            }
            //to newton force
            if args[0] == "tonf" || args[1] == "TN" {
                vptr::print_nm(&args);
                line.truncate(0);
                continue;
            }

            if args[0] == "pps" {
                vptr::print_ps(&args);
                line.truncate(0);
                continue;
            }

            if args[0] == "pbhp" || args[1] == "HP" {
                vptr::print_pbhp(&args);
                line.truncate(0);
                continue;
            }

            if args[0] == "tolitres" {
                vptr::print_litres(&args);
                line.truncate(0);
                continue;
            }

            if args[0] == "toefsf" {
                vptr::prinf_efsf(&args);
                line.truncate(0);
                continue;
            }

            if args[0] == "tobb" {
                vptr::prinf_bluebberies(&args);
                line.truncate(0);
                continue;
            }

            println!("unknown command");
            line.truncate(0);
            continue;
        }
    }
}
