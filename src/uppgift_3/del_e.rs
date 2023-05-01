use super::prelude::*;

pub async fn uppgift_e() {
    ensure_dir_exists("uppgifter/3/e").await;

    /*
    theoretical: critical when gamma = omega -->
    r/2m = sqrt(k/m) -->
    r = 2m * sqrt(k/m)
    r = 2 * 100.0
     */

    let r = 2.0 * DEFAULT_INIT_SNAPSHOT.mass * (DEFAULT_K / DEFAULT_INIT_SNAPSHOT.mass).sqrt();

    let values = vec![18.0, 18.5, 19.0, 19.5, 20.0, 21.0, 22.0];

    let tasks: Vec<_> = values
        .into_iter()
        .map(|r| {
            tokio::spawn(async move {
                let mut output = File::create(&format!("uppgifter/3/e/r-{:.2}.csv", r))
                    .await
                    .unwrap();
                uppgift3_run_simulation(
                    DEFAULT_INIT_SNAPSHOT,
                    DEFAULT_K,
                    r,
                    0.0001,
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
