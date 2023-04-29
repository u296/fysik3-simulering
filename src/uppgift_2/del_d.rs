use super::prelude::*;

pub async fn uppgift_d() {
    ensure_dir_exists("uppgifter/2/d").await;

    let values = vec![-10.0, -5.0, 0.0, 5.0, 10.0];

    for init_velocity in values {
        let mut output_file =
            File::create(&format!("uppgifter/2/d/init-velocity-{init_velocity}.csv"))
                .await
                .unwrap();

        let snapshot = FreeFallObjectSnapshot {
            velocity: vector![0.0, init_velocity],
            ..*DEFAULT_BALL
        };
        uppgift2_run_simulation(snapshot, DEFAULT_R, HONEY_RHO, 0.001, &mut output_file).await;
    }
}
