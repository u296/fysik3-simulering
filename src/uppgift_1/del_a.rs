use super::prelude::*;

pub async fn uppgift_a() {
    ensure_dir_exists("uppgifter/1/a").await;

    let mut output_file = File::create("uppgifter/1/a/result.csv").await.unwrap();
    uppgift1_run_simulation(
        *super::BALL_SNAPSHOT,
        *super::BALL_AIR_RESISTANCE,
        0.001,
        &mut output_file,
    )
    .await;
}
