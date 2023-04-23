use super::{prelude::*, AIRCRAFT_RESISTANCE, AIRCRAFT_SNAPSHOT};

pub fn uppgift_f() {
    ensure_dir_exists("uppgifter/1/f");
    let mut output_file = File::create("uppgifter/1/f/result.csv").unwrap();

    let air_resistance = AirResistanceParameters {
        c_d: 0.0,
        ..*AIRCRAFT_RESISTANCE
    };

    run_simulation(*AIRCRAFT_SNAPSHOT, air_resistance, 0.01, &mut output_file)
}
