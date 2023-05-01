use super::{prelude::*, DEFAULT_BALL_RADIUS};

pub async fn uppgift_e() {
    ensure_dir_exists("uppgifter/2/e").await;
    let mut output_file = File::create("uppgifter/2/e/result.csv").await.unwrap();
    uppgift2_run_simulation(
        *DEFAULT_BALL,
        oil_r(DEFAULT_BALL_RADIUS),
        OIL_RHO,
        0.001,
        &mut output_file,
    )
    .await;
}
