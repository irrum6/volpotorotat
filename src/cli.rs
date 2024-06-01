pub mod cli {
    use crate::vptr;
    use crate::VERSION;

    pub fn run_cli(args: &Vec<String>) {

        // let cli_args;
        if args[1] == "--version" || args[1] == "-V" {
            println!("Volpotorotat v{}", VERSION);
            return;
        }
        if args[0] == "vol" {
            vptr::print_vol(&args);
        }

        if args[0] == "tolbf" || args[1] == "TL" {
            vptr::print_lbf(&args);
        }
        //to newton force
        if args[0] == "tonf" || args[1] == "TN" {
            vptr::print_nm(&args);
        }

        if args[0] == "pps" {
            vptr::print_ps(&args);
        }

        if args[0] == "pbhp" || args[1] == "HP" {
            vptr::print_pbhp(&args);
        }
    }
}
