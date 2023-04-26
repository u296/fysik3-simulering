use super::{prelude::*, DEFAULT_BALL_RADIUS};

pub async fn uppgift_e() {
    ensure_dir_exists("uppgifter/2/e").await;
    let mut output_file = File::create("uppgifter/2/e/result.csv").await.unwrap();
    run_simulation(
        *DEFAULT_BALL,
        0.9 * DEFAULT_BALL_RADIUS,
        918.0,
        0.001,
        &mut output_file,
    )
    .await;
}
