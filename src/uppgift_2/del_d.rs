use super::prelude::*;

pub async fn uppgift_d() {
    let start = Instant::now();

    ensure_dir_exists("uppgifter/2/d");

    let values = vec![-10.0, -5.0, 0.0, 5.0, 10.0];

    for init_velocity in values {
        let mut output_file =
            File::create(&format!("uppgifter/2/d/init-velocity-{init_velocity}.csv")).unwrap();

        let snapshot = FreeFallObjectSnapshot {
            velocity: vector![0.0, init_velocity],
            ..*DEFAULT_BALL
        };
        run_simulation(snapshot, DEFAULT_R, HONEY_RHO, 0.001, &mut output_file);
    }

    println!(
        "2-d finished in {}",
        Instant::now().duration_since(start).as_secs_f64()
    );
}
