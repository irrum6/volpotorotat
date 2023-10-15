use volpotorotat::vptr::tools::tools::volume;

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    let version = "0.1";

    if args.len() < 2 {
        return;
    }

    if args[1] == "--version" || args[1] == "-V" {
        println!("Volpotorotat v{}", version);
        return;
    }
    if args.len() < 4 {
        println!("pass enough parameters to calculate");
        return;
    }

    let bore: f64 = args[1].trim().parse().expect("number");
    let stroke: f64 = args[2].trim().parse().expect("number");

    let cylinders: u8 = args[3].trim().parse().expect("number");

    let volume = volume(bore, stroke, cylinders);

    println!("volume is {}", volume);
}
