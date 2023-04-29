use super::prelude::*;

pub async fn uppgift_a() {
    ensure_dir_exists("uppgifter/2/a").await;
    let mut output_file = File::create("uppgifter/2/a/result.csv").await.unwrap();
    uppgift2_run_simulation(*DEFAULT_BALL, DEFAULT_R, HONEY_RHO, 0.01, &mut output_file).await;
}
