pub mod convert;
pub mod tools;
pub mod vptr {
    pub static VERSION: &str = "0.3.2";
    pub use super::convert;
    pub use super::tools;

    //from_repl has one less parameter
    pub fn print_vol(v: &Vec<String>) {
        if v.len() < 4 {
            println!("pass enough parameters to calculate");
            return;
        }
        let bore: f64 = v[1].trim().parse().expect("number");
        let stroke: f64 = v[2].trim().parse().expect("number");
        let cylinders: u8 = v[3].trim().parse().expect("number");
        let volume = tools::tools::volume(bore, stroke, cylinders);

        println!("volume is {}", volume);
        return;
    }

    pub fn print_lbf(v: &Vec<String>) {
        if v.len() < 2 {
            println!("pass enough parameters to calculate");
            return;
        }

        let force: f64 = v[1].trim().parse().expect("number");

        let lbf = convert::convert::nm_lbft(force);
        println!(
            "{} newton-meters torque are equal to {:.5} pound foots",
            force, lbf
        );
        return;
    }

    pub fn print_nm(v: &Vec<String>) {
        if v.len() < 2 {
            println!("pass enough parameters to calculate");
            return;
        }
        let pound_force: f64 = v[1].trim().parse().expect("number");

        let newtons = convert::convert::lbft_nm(pound_force);
        println!(
            "{} pounds foot torque are equal to {:.5} newton-meters ",
            pound_force, newtons
        );
        return;
    }

    pub fn print_ps(v: &Vec<String>) {
        if v.len() < 3 {
            println!("pass enough parameters to calculate");
            return;
        }
        let torque: f64 = v[1].trim().parse().expect("number");
        let rpm: u16 = v[2].trim().parse().expect("number");

        let power_ps = tools::tools::power_ps(torque, rpm);
        println!(
            "{} Newton-meters torque at {} rpm generates {:.5} horse power(PS)",
            torque, rpm, power_ps
        );
        return;
    }
    //print power bhp
    pub fn print_pbhp(v: &Vec<String>) {
        if v.len() < 3 {
            println!("pass enough parameters to calculate");
            return;
        }
        let torque: f64 = v[1].trim().parse().expect("number");
        let rpm: u16 = v[2].trim().parse().expect("number");

        let power_bhp = tools::tools::power_hp2(torque, rpm);
        println!(
            "{} pound-foot torque at {} rpm generates {:.5} horse power",
            torque, rpm, power_bhp
        );
        return;
    }
    pub fn print_litres(v: &Vec<String>) {
        if v.len() < 2 {
            println!("pass enough parameters to calculate");
            return;
        }
        let mpg: u8 = v[1].trim().parse().expect("small enought number <256");
        let litres = convert::convert::mpg_to_litres(mpg);
        println!(
            "{} miles per gallon  equals {:.3} litres per 100 kilometer",
            mpg, litres
        );
    }
    pub fn prinf_efsf(v: &Vec<String>) {
        if v.len() < 2 {
            println!("pass enough parameters to calculate");
            return;
        }
        let pascals: f64 = v[1].trim().parse().expect("number float64");
        let efsf = convert::convert::to_efsf_reduced(pascals);
        println!(
            "{} pascals equal to  {:.5} elefant forces per square foot",
            pascals, efsf
        );
    }

    pub fn prinf_bluebberies(v: &Vec<String>) {
        if v.len() < 2 {
            println!("pass enough parameters to calculate");
            return;
        }
        let cc: f64 = v[1].trim().parse().expect("number float64");
        let bb = convert::convert::to_blueberries(cc);
        println!("{} cubic centimetres equal to  {:.6} blueberries", cc, bb);
    }
}
