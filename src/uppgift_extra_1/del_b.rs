use super::prelude::*;

pub async fn uppgift_b() {
    ensure_dir_exists("uppgifter/extra_1/b").await;

    let values = vec![0.0001, 0.001, 0.01, 0.1, 1.0];

    let tasks: Vec<_> = values
        .into_iter()
        .map(|dt| {
            tokio::spawn(async move {
                let mut output = File::create(&format!("uppgifter/extra_1/b/dt-{dt}.csv"))
                    .await
                    .unwrap();
                uppgift_extra_1_run_simulation(
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
