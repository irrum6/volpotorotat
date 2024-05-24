pub mod convert;
pub mod tools;
pub mod vptr {
    pub static VERSION: &str = "0.3.1";
    pub use super::convert;
    pub use super::tools;

    //from_repl has one less parameter
    pub fn print_vol(v: &Vec<String>, from_repl: bool) {
        if (v.len() < 5 && !from_repl) || (from_repl && v.len() < 4) {
            println!("pass enough parameters to calculate");
            return;
        }
        if from_repl {
            let bore: f64 = v[1].trim().parse().expect("number");
            let stroke: f64 = v[2].trim().parse().expect("number");
            let cylinders: u8 = v[3].trim().parse().expect("number");
            let volume = tools::tools::volume(bore, stroke, cylinders);

            println!("volume is {}", volume);
            return;
        }
        let bore: f64 = v[2].trim().parse().expect("number");
        let stroke: f64 = v[3].trim().parse().expect("number");

        let cylinders: u8 = v[4].trim().parse().expect("number");

        let volume = tools::tools::volume(bore, stroke, cylinders);

        println!("volume is {}", volume);
    }

    pub fn print_lbf(v: &Vec<String>, from_repl: bool) {
        if (v.len() < 3 && !from_repl) || (from_repl && v.len() < 2) {
            println!("pass enough parameters to calculate");
            return;
        }

        if from_repl {
            let force: f64 = v[1].trim().parse().expect("number");

            let lbf = convert::convert::nm_lbft(force);
            println!(
                "{} newton-meters torque are equal to {:.5} pound foots",
                force, lbf
            );
            return;
        }
        let force: f64 = v[2].trim().parse().expect("number");

        let lbf = convert::convert::nm_lbft(force);
        println!(
            "{} newton-meters torque are equal to {:.5} pound foots",
            force, lbf
        );
    }

    pub fn print_nm(v: &Vec<String>, from_repl: bool) {
        if (v.len() < 3 && !from_repl) || (from_repl && v.len() < 2) {
            println!("pass enough parameters to calculate");
            return;
        }
        if from_repl {
            let pound_force: f64 = v[1].trim().parse().expect("number");

            let newtons = convert::convert::lbft_nm(pound_force);
            println!(
                "{} pounds foot torque are equal to {:.5} newton-meters ",
                pound_force, newtons
            );
            return;
        }
        let pound_force: f64 = v[2].trim().parse().expect("number");

        let newtons = convert::convert::lbft_nm(pound_force);
        println!(
            "{} pounds foot torque are equal to {:.5} newton-meters ",
            pound_force, newtons
        );
    }

    pub fn print_ps(v: &Vec<String>, from_repl: bool) {
        if (v.len() < 4 && !from_repl) || (from_repl && v.len() < 3) {
            println!("pass enough parameters to calculate");
            return;
        }
        if from_repl {
            let torque: f64 = v[1].trim().parse().expect("number");
            let rpm: u16 = v[2].trim().parse().expect("number");

            let power_ps = tools::tools::power_ps(torque, rpm);
            println!(
                "{} Newton-meters torque at {} rpm generates {:.5} horse power(PS)",
                torque, rpm, power_ps
            );
            return;
        }
        let torque: f64 = v[2].trim().parse().expect("number");
        let rpm: u16 = v[3].trim().parse().expect("number");

        let power_ps = tools::tools::power_ps(torque, rpm);
        println!(
            "{} Newton-meters torque at {} rpm generates {:.5} horse power(PS)",
            torque, rpm, power_ps
        );
    }
    //print power bhp
    pub fn print_pbhp(v: &Vec<String>, from_repl: bool) {
        if (v.len() < 4 && !from_repl) || (from_repl && v.len() < 3) {
            println!("pass enough parameters to calculate");
            return;
        }
        if from_repl {
            let torque: f64 = v[1].trim().parse().expect("number");
            let rpm: u16 = v[2].trim().parse().expect("number");

            let power_bhp = tools::tools::power_hp2(torque, rpm);
            println!(
                "{} pound-foot torque at {} rpm generates {:.5} horse power",
                torque, rpm, power_bhp
            );
            return;
        }
        let torque: f64 = v[2].trim().parse().expect("number");
        let rpm: u16 = v[3].trim().parse().expect("number");

        let power_bhp = tools::tools::power_hp2(torque, rpm);
        println!(
            "{} pound-foot torque at {} rpm generates {:.5} horse power",
            torque, rpm, power_bhp
        );
    }
}
