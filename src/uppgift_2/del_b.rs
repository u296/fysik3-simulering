use super::prelude::*;

pub async fn uppgift_b() {
    ensure_dir_exists("uppgifter/2/b").await;

    let values = vec![0.001, 0.005, 0.01, 0.05];

    for dt in values {
        let mut output_file = File::create(&format!("uppgifter/2/b/dt-{:.3}.csv", dt))
            .await
            .unwrap();
        uppgift2_run_simulation(
            *DEFAULT_BALL,
            honey_r(DEFAULT_BALL_RADIUS),
            HONEY_RHO,
            dt,
            &mut output_file,
        )
        .await;
    }
}
