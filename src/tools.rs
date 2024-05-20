pub mod tools {
    use std::f64::consts::PI;
    // cylinders - cylinder count
    // dimensions are given in millimeer
    // volume is calculated in cubic centimeter
    pub fn volume(bore: f64, stroke: f64, cylinders: u8) -> f64 {
        return (PI * bore * bore * stroke * (cylinders as f64)) / 4000.0;
    }
    //torque nm
    pub fn power_watts(torque: f64, rpm: u16) -> f64 {
        return (torque * 2.0 * PI * rpm as f64) / 60.0;
    }
    //torque nm
    pub fn power_hp(torque: f64, rpm: u16) -> f64 {
        return (torque * 2.0 * PI * rpm as f64) / (60.0 * 745.69987);
    }
    //torque nm
    pub fn power_ps(torque: f64, rpm: u16) -> f64 {
        return (torque * 2.0 * PI * rpm as f64) / (60.0 * 735.49875);
    }
    /**
     * torque lb-ft
     * rpm
     * returns brake (imperial) horse power (745.7 watts)
     */
    pub fn power_hp2(torque: f64, rpm: u16) -> f64 {
        return (torque * 2.0 * PI * rpm as f64) / (33000 as f64);
    }
    //power in watts
    //out in newton-meters
    pub fn torque_nm(power: f64, rpm: u16) -> f64 {
        return (power * 60.0) / (PI * 2.0 * rpm as f64);
    }
}
