use volpotorotat::vptr::convert::convert;
use volpotorotat::vptr::tools::tools;

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    let version = "0.2.1";

    if args.len() < 2 {
        return;
    }

    if args[1] == "--version" || args[1] == "-V" {
        println!("Volpotorotat v{}", version);
        return;
    }

    if args[1] == "vol" {
        if args.len() < 5 {
            println!("pass enough parameters to calculate");
            return;
        }
        let bore: f64 = args[2].trim().parse().expect("number");
        let stroke: f64 = args[3].trim().parse().expect("number");

        let cylinders: u8 = args[4].trim().parse().expect("number");

        let volume = tools::volume(bore, stroke, cylinders);

        println!("volume is {}", volume);
    }

    if args[1] == "tolbf" {
        if args.len() < 3 {
            println!("pass enough parameters to calculate");
            return;
        }
        let force: f64 = args[2].trim().parse().expect("number");

        let lbf = convert::newtons_lbf(force);
        println!("{} Newtons are equal to {:.5} pounds force", force, lbf);
    }
}
