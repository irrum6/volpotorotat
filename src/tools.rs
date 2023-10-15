pub mod tools {
    use std::f64::consts::PI;
    // cylinders - cylinder count
    // dimensions are given in millimeer
    // volume is calculated in cubic centimeter
    pub fn volume(bore: f64, stroke: f64, cylinders: u8) -> f64 {
        return (PI * bore * bore * stroke * (cylinders as f64)) / 4000.0;
    }

    pub fn power_watts(torque: f64, rpm: u16) -> f64 {
        return (torque * PI * rpm as f64) / 60.0;
    }

    pub fn power_hp(torque: f64, rpm: u16) -> f64 {
        return (torque * PI * rpm as f64) / (60.0 * 745.69987);
    }
    pub fn power_ps(torque: f64, rpm: u16) -> f64 {
        return (torque * PI * rpm as f64) / (60.0 * 735.49875);
    }

    //power in watts
    //out in newton-meters
    pub fn torque_nm(power: f64, rpm: u16) -> f64 {
        return (power * 60.0) / (PI * rpm as f64);
    }
}