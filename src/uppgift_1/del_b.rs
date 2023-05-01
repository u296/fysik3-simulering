use super::prelude::*;

pub async fn uppgift_b() {
    ensure_dir_exists("uppgifter/1/b").await;

    let vals = vec![0.0001, 0.001, 0.01, 0.05, 0.1, 0.25];

    for dt in vals {
        let mut output_file = File::create(&format!("uppgifter/1/b/dt-{:.4}.csv", dt))
            .await
            .unwrap();
        uppgift1_run_simulation(
            *super::BALL_SNAPSHOT,
            *super::BALL_AIR_RESISTANCE,
            dt,
            &mut output_file,
        )
        .await;
    }
}
