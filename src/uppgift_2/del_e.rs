use super::{prelude::*, DEFAULT_BALL_RADIUS};

pub async fn uppgift_e() {
    let start = Instant::now();
    ensure_dir_exists("uppgifter/2/e");
    let mut output_file = File::create("uppgifter/2/e/result.csv").unwrap();
    run_simulation(
        *DEFAULT_BALL,
        0.9 * DEFAULT_BALL_RADIUS,
        918.0,
        0.001,
        &mut output_file,
    );

    println!(
        "2-e finished in {}",
        Instant::now().duration_since(start).as_secs_f64()
    );
}
