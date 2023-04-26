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

    let values = vec![r - 2.0, r - 1.0, r, r + 1.0, r + 2.0];

    let tasks: Vec<_> = values
        .into_iter()
        .map(|r| {
            tokio::spawn((move || async move {
                let mut output = File::create(&format!("uppgifter/3/e/r-{r}.csv"))
                    .await
                    .unwrap();
                run_simulation(
                    DEFAULT_INIT_SNAPSHOT,
                    DEFAULT_K,
                    r,
                    0.001,
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
