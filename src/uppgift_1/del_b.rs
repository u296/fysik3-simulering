use super::prelude::*;

pub fn uppgift_b() {
    ensure_dir_exists("uppgifter/1/b");

    let vals = vec![0.01, 0.05, 0.1, 0.25, 0.5, 1.0];

    for dt in vals {
        let mut output_file = File::create(&format!("uppgifter/1/b/dt-{dt}.csv")).unwrap();
        run_simulation(
            *super::BALL_SNAPSHOT,
            *super::BALL_AIR_RESISTANCE,
            dt,
            &mut output_file,
        );
    }
}
