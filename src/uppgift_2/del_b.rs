use super::prelude::*;

pub fn uppgift_b() {
    ensure_dir_exists("uppgifter/2/b");

    let values = vec![0.001, 0.005, 0.01, 0.05, 0.1];

    for dt in values {
        let mut output_file = File::create(&format!("uppgifter/2/b/dt-{dt}.csv")).unwrap();
        run_simulation(*DEFAULT_BALL, DEFAULT_R, HONEY_RHO, 0.01, &mut output_file);
    }
}
