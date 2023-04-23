use super::{prelude::*, AIRCRAFT_RESISTANCE, AIRCRAFT_SNAPSHOT};

pub fn uppgift_g() {
    ensure_dir_exists("uppgifter/1/g");

    let vals = vec![0.001, 0.005, 0.01, 0.05, 0.1];

    let air_resistance = AirResistanceParameters {
        c_d: 0.0,
        ..*AIRCRAFT_RESISTANCE
    };

    for dt in vals {
        let mut output_file = File::create(&format!("uppgifter/1/g/dt-{dt}.csv")).unwrap();
        run_simulation(*AIRCRAFT_SNAPSHOT, air_resistance, dt, &mut output_file)
    }
}
