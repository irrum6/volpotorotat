pub mod convert {
    //pound force
    pub fn lbf_newtons(force: f64) -> f64 {
        return force * 0.45359237 * 9.8066;
    }
    pub fn newtons_lbf(force: f64) -> f64 {
        return force / (9.8066 * 0.45359237);
    }
    //torque pound foot to newtons-meters
    pub fn lbft_nm(lbft: f64) -> f64 {
        return lbft * 0.45359237 * 9.8066 * 0.3048;
    }
    //torque newton-meters to pound foott
    pub fn nm_lbft(nm: f64) -> f64 {
        return nm / (0.45359237 * 9.8066 * 0.3048);
    }

    pub fn ftlb_jouls(energy: f64) -> f64 {
        return energy * 0.3048 * 0.45359237 * 9.8066;
    }

    pub fn jouls_ftlb(energy: f64) -> f64 {
        return ((energy / 9.81) / 0.3048) / 0.45359237;
    }
}
