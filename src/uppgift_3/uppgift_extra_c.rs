use super::prelude::*;

pub async fn uppgift_extra_c() {
    ensure_dir_exists("uppgifter/3/extra/c").await;

    {
        let mut output = File::create("uppgifter/3/extra/c/euler_cromer.csv")
            .await
            .unwrap();
        run_simulation(
            DEFAULT_INIT_SNAPSHOT,
            DEFAULT_K,
            0.0,
            0.001,
            &mut output,
            EulerCromerSolver::new,
        )
        .await;
    }

    // euler-cromer is (somewhat) stable at dt = 0.1, but euler is not at dt = 0.01
    {
        let mut output = File::create("uppgifter/3/extra/c/euler.csv").await.unwrap();
        run_simulation(
            DEFAULT_INIT_SNAPSHOT,
            DEFAULT_K,
            0.0,
            0.001,
            &mut output,
            EulerSolver::new,
        )
        .await;
    }
}
