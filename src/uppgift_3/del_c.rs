use super::prelude::*;

pub async fn uppgift_c() {
    ensure_dir_exists("uppgifter/3/c").await;

    let mut output = File::create("uppgifter/3/c/result.csv").await.unwrap();
    uppgift3_run_simulation(
        DEFAULT_INIT_SNAPSHOT,
        DEFAULT_K,
        1.0,
        0.01,
        &mut output,
        EulerCromerSolver::new,
    )
    .await;
}
