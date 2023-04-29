use super::prelude::*;

pub async fn uppgift_d() {
    ensure_dir_exists("uppgifter/3/d").await;

    {
        let mut output = File::create("uppgifter/3/d/svag.csv").await.unwrap();
        uppgift3_run_simulation(
            DEFAULT_INIT_SNAPSHOT,
            DEFAULT_K,
            2.0,
            0.01,
            &mut output,
            EulerCromerSolver::new,
        )
        .await;
    }

    // theoretical critical: r = 20.0 = 2m * sqrt(k/m)

    {
        let mut output = File::create("uppgifter/3/d/stark.csv").await.unwrap();
        uppgift3_run_simulation(
            DEFAULT_INIT_SNAPSHOT,
            DEFAULT_K,
            50.0,
            0.01,
            &mut output,
            EulerCromerSolver::new,
        )
        .await;
    }
}
