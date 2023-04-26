use super::prelude::*;

pub async fn uppgift_c() {
    let init_snapshot = FreeFallObjectSnapshot {
        mass: 1.0,
        charge: 0.0,
        frontal_area: 0.0,
        volume: 0.0,
        position: vector![10.0, 0.0],
        velocity: vector![0.0, 0.0],
    };

    ensure_dir_exists("uppgifter/3/c").await;

    let mut output = File::create("uppgifter/3/c/results.csv").await.unwrap();
    run_simulation(init_snapshot, 100.0, 1.0, 0.01, &mut output).await;
}
