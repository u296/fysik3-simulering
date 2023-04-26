use super::prelude::*;

pub async fn uppgift_extra_b() {
    ensure_dir_exists("uppgifter/3/extra/b").await;

    let values = vec![0.0001, 0.001, 0.01, 0.1, 1.0];

    let tasks: Vec<_> = values
        .into_iter()
        .map(|dt| {
            tokio::spawn((move || async move {
                let mut output = File::create(&format!("uppgifter/3/extra/b/dt-{dt}.csv"))
                    .await
                    .unwrap();
                run_simulation(
                    DEFAULT_INIT_SNAPSHOT,
                    DEFAULT_K,
                    0.0,
                    dt,
                    &mut output,
                    EulerCromerSolver::new,
                )
                .await;
            })())
        })
        .collect();

    for task in tasks {
        task.await.unwrap();
    }
}
