use super::prelude::*;

pub fn uppgift_a() {
    ensure_dir_exists("uppgifter/2/a");
    let mut output_file = File::create("uppgifter/2/a/result.csv").unwrap();
    run_simulation(*DEFAULT_BALL, DEFAULT_R, HONEY_RHO, 0.01, &mut output_file);
}
