use super::{prelude::*, AIRCRAFT_RESISTANCE, AIRCRAFT_SNAPSHOT};

pub fn uppgift_d() {
    ensure_dir_exists("uppgifter/1/d");
    let mut output_file = File::create("uppgifter/1/d/result.csv").unwrap();
    run_simulation(
        AIRCRAFT_SNAPSHOT,
        AIRCRAFT_RESISTANCE,
        0.01,
        &mut output_file,
    )
}
