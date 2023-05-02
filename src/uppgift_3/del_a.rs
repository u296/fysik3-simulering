use super::prelude::*;

pub async fn uppgift_a() {
    ensure_dir_exists("uppgifter/3/a").await;

    let mut output = File::create("uppgifter/3/a/result.csv").await.unwrap();
    uppgift3_run_simulation(
        DEFAULT_INIT_SNAPSHOT,
        DEFAULT_K,
        0.0,
        0.01,
        &mut output,
        EulerCromerSolver::new,
    )
    .await;
}
