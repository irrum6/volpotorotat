use volpotorotat::vptr::convert::convert;
use volpotorotat::vptr::tools::tools;

pub fn print_vol(v: &Vec<String>) {
    if v.len() < 5 {
        println!("pass enough parameters to calculate");
        return;
    }
    let bore: f64 = v[2].trim().parse().expect("number");
    let stroke: f64 = v[3].trim().parse().expect("number");

    let cylinders: u8 = v[4].trim().parse().expect("number");

    let volume = tools::volume(bore, stroke, cylinders);

    println!("volume is {}", volume);
}

pub fn print_lbf(v: &Vec<String>) {
    if v.len() < 3 {
        println!("pass enough parameters to calculate");
        return;
    }
    let force: f64 = v[2].trim().parse().expect("number");

    let lbf = convert::nm_lbft(force);
    println!(
        "{} newton-meters torque are equal to {:.5} pound foots",
        force, lbf
    );
}

pub fn print_nm(v: &Vec<String>) {
    if v.len() < 3 {
        println!("pass enough parameters to calculate");
        return;
    }
    let pound_force: f64 = v[2].trim().parse().expect("number");

    let newtons = convert::lbft_nm(pound_force);
    println!(
        "{} pounds foot torque are equal to {:.5} newton-meters ",
        pound_force, newtons
    );
}

pub fn print_ps(v: &Vec<String>) {
    if v.len() < 4 {
        println!("pass enough parameters to calculate");
        return;
    }
    let torque: f64 = v[2].trim().parse().expect("number");
    let rpm: u16 = v[3].trim().parse().expect("number");

    let power_ps = tools::power_ps(torque, rpm);
    println!(
        "{} Newtons torque at {} rpm generates {:.5} horse power(PS)",
        torque, rpm, power_ps
    );
}
//print power bhp
pub fn print_pbhp(v: &Vec<String>) {
    if v.len() < 4 {
        println!("pass enough parameters to calculate");
        return;
    }
    let torque: f64 = v[2].trim().parse().expect("number");
    let rpm: u16 = v[3].trim().parse().expect("number");

    let power_bhp = tools::power_hp2(torque, rpm);
    println!(
        "{} pound-foot torque at {} rpm generates {:.5} horse power",
        torque, rpm, power_bhp
    );
}
fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    const VERSION: &str = "0.2.5";

    if args.len() < 2 {
        return;
    }

    if args[1] == "--version" || args[1] == "-V" {
        println!("Volpotorotat v{}", VERSION);
        return;
    }

    if args[1] == "vol" {
        print_vol(&args);
    }

    if args[1] == "tolbf" || args[1] == "TL" {
        print_lbf(&args);
    }
    //to newton force
    if args[1] == "tonf" || args[1] == "TN" {
        print_nm(&args);
    }

    if args[1] == "pps" {
        print_ps(&args);
    }

    if args[1] == "pbhp" || args[1] == "HP" {
        print_pbhp(&args);
    }
}
