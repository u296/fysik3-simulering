use super::prelude::*;

pub async fn uppgift_a() {
    ensure_dir_exists("uppgifter/2/a").await;
    let mut output_file = File::create("uppgifter/2/a/result.csv").await.unwrap();
    uppgift2_run_simulation(
        *DEFAULT_BALL,
        honey_r(DEFAULT_BALL_RADIUS),
        HONEY_RHO,
        0.005,
        &mut output_file,
    )
    .await;
}
