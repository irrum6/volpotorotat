pub mod convert {
    const FOOT: f64 = 0.3048;
    //1760 feets
    const MILE: f64 = 1609.344;
    //free fall
    const GFREEFALL: f64 = 9.80665;
    //0.45359237 * 9.80665;
    const POUND_FORCE: f64 = 4.4482216152605;
    //0.45359237 * 0.3048 * 9.80665
    const POUND_FOOT_TORQUE: f64 = 1.35581794833140040;

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

    pub fn mph_to_kmh(miles_per_hour: u32) -> f64 {
        return miles_per_hour as f64 * MILE;
    }
    /**
     * 
     */
    pub fn kmh_to_mph(kilometers_hour: u32) -> f64 {
        return kilometers_hour as f64 / MILE;
    }
    /**
     * from miles per gallon to litres per 100 kilometer
     * more than 256 miles per gallon is unlikely
     */
    pub fn mpg_to_litres(mpg: u8) -> f32 {
        //US gallon 3.785411784 gal
        //Mile 1.609344
        //miles m
        //100 / (m * M /gal) =
        //100 gal /(m*m)
        //define
        //100 gal/M
        // 100G/M * 1/m
        // 100G/M / m
        // 3785411784 / 1609344 = 2352.1458333333
        // mpg_to_lp100km = m=> 235.21458333333 / m;
        return 235.21458333333 / (mpg as f32);
    }
}
