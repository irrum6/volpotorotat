use volpotorotat::vptr::convert::convert;
use volpotorotat::vptr::tools::tools;

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    const VERSION: &str = "0.2.3";

    if args.len() < 2 {
        return;
    }

    if args[1] == "--version" || args[1] == "-V" {
        println!("Volpotorotat v{}", VERSION);
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

    if args[1] == "tolbf" || args[1] == "TL" {
        if args.len() < 3 {
            println!("pass enough parameters to calculate");
            return;
        }
        let force: f64 = args[2].trim().parse().expect("number");

        let lbf = convert::nm_lbft(force);
        println!("{} newton-meters torque are equal to {:.5} pound foots", force, lbf);
    }
    //to newton force
    if args[1] == "tonf" || args[1] == "TN" {
        if args.len() < 3 {
            println!("pass enough parameters to calculate");
            return;
        }
        let pound_force: f64 = args[2].trim().parse().expect("number");

        let newtons = convert::lbft_nm(pound_force);
        println!(
            "{} pounds foot torque are equal to {:.5} newton-meters ",
            pound_force, newtons
        );
    }

    if args[1] == "pps" {
        if args.len() < 4 {
            println!("pass enough parameters to calculate");
            return;
        }
        let torque: f64 = args[2].trim().parse().expect("number");
        let rpm: u16 = args[3].trim().parse().expect("number");

        let power_ps = tools::power_ps(torque, rpm);
        println!(
            "{} Newtons force at {} rpm generates {:.5} horse power(PS)",
            torque, rpm, power_ps
        );
    }
}
