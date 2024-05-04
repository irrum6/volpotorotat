pub mod convert {
    static FOOT: f64 = 0.3048;
    static GFREEFALL: f64 = 9.80665;
    static POUND_FORCE: f64 = 0.45359237 * 9.80665;
    static POUND_FOOT_TORQUE: f64 = 0.45359237 * 0.3048 * 9.80665;

    //pound force
    pub fn lbf_newtons(force: f64) -> f64 {
        return force * POUND_FORCE;
    }
    pub fn newtons_lbf(force: f64) -> f64 {
        return force / (POUND_FORCE);
    }
    //torque pound foot to newtons-meters
    pub fn lbft_nm(lbft: f64) -> f64 {
        return lbft * POUND_FOOT_TORQUE;
    }
    //torque newton-meters to pound foott
    pub fn nm_lbft(nm: f64) -> f64 {
        return nm / (POUND_FOOT_TORQUE);
    }

    pub fn ftlb_jouls(energy: f64) -> f64 {
        return energy * 0.3048 * 0.45359237 * 9.8066;
    }

    pub fn jouls_ftlb(energy: f64) -> f64 {
        return ((energy / 9.81) / 0.3048) / 0.45359237;
    }
}
