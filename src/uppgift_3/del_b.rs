use super::prelude::*;

pub async fn uppgift_b() {
    ensure_dir_exists("uppgifter/3/b").await;

    let vals = vec![0.1, 0.05, 0.01, 0.005, 0.001];

    let tasks: Vec<_> = vals
        .into_iter()
        .map(|dt| {
            tokio::spawn(async move {
                let mut output = File::create(&format!("uppgifter/3/b/dt-{:.3}.csv", dt))
                    .await
                    .unwrap();
                uppgift3_run_simulation(
                    DEFAULT_INIT_SNAPSHOT,
                    DEFAULT_K,
                    0.0,
                    dt,
                    &mut output,
                    EulerCromerSolver::new,
                )
                .await;
            })
        })
        .collect();

    for task in tasks {
        task.await.unwrap();
    }
}
