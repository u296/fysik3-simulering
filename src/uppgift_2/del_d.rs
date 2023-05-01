use super::prelude::*;

pub async fn uppgift_d() {
    ensure_dir_exists("uppgifter/2/d").await;

    let values = vec![-1.0, -0.5, 0.0, 0.5, 1.0];

    for init_velocity in values {
        let mut output_file = File::create(&format!(
            "uppgifter/2/d/init-velocity-{:.1}.csv",
            init_velocity
        ))
        .await
        .unwrap();

        let snapshot = BodySnapshot {
            velocity: vector![0.0, init_velocity],
            ..*DEFAULT_BALL
        };
        uppgift2_run_simulation(
            snapshot,
            honey_r(DEFAULT_BALL_RADIUS),
            HONEY_RHO,
            0.001,
            &mut output_file,
        )
        .await;
    }
}
