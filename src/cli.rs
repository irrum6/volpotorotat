pub mod cli {
    use crate::vptr;
    use crate::VERSION;

    pub fn run_cli(args: &Vec<String>) {
        if args[1] == "--version" || args[1] == "-V" {
            println!("Volpotorotat v{}", VERSION);
            return;
        }
        if args[1] == "vol" {
            vptr::print_vol(&args, false);
        }

        if args[1] == "tolbf" || args[1] == "TL" {
            vptr::print_lbf(&args, false);
        }
        //to newton force
        if args[1] == "tonf" || args[1] == "TN" {
            vptr::print_nm(&args, false);
        }

        if args[1] == "pps" {
            vptr::print_ps(&args, false);
        }

        if args[1] == "pbhp" || args[1] == "HP" {
            vptr::print_pbhp(&args, false);
        }
    }
}
