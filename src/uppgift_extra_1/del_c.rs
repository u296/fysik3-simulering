use super::prelude::*;

pub async fn uppgift_c() {
    ensure_dir_exists("uppgifter/extra_1/c").await;

    let mut tasks = vec![];

    tasks.push(tokio::spawn(async {
        let mut output = File::create("uppgifter/extra_1/c/euler_cromer.csv")
            .await
            .unwrap();
        uppgift_extra_1_run_simulation(
            DEFAULT_INIT_SNAPSHOT,
            DEFAULT_K,
            0.0,
            0.001,
            &mut output,
            EulerCromerSolver::new,
        )
        .await;
    }));

    // euler-cromer is (somewhat) stable at dt = 0.1, but euler is not at dt = 0.01
    tasks.push(tokio::spawn(async {
        let mut output = File::create("uppgifter/extra_1/c/euler.csv").await.unwrap();
        uppgift_extra_1_run_simulation(
            DEFAULT_INIT_SNAPSHOT,
            DEFAULT_K,
            0.0,
            0.001,
            &mut output,
            EulerSolver::new,
        )
        .await;
    }));

    for task in tasks {
        task.await.unwrap();
    }
}
