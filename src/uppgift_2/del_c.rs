use super::prelude::*;

/*
   running this at low tickrates causes severe instability
*/
pub async fn uppgift_c() {
    let mut tasks = Vec::new();

    {
        // mass variation
        ensure_dir_exists("uppgifter/2/c/mass").await;

        let values = vec![0.001, 0.01, 0.1, 0.5, 1.0, 2.0];

        tasks.extend(values.into_iter().map(|mass| {
            tokio::spawn(async move {
                let initial = FreeFallObjectSnapshot {
                    mass,
                    ..*DEFAULT_BALL
                };

                let mut output_file =
                    File::create(&format!("uppgifter/2/c/mass/mass-{:.3}.csv", mass))
                        .await
                        .unwrap();
                uppgift2_run_simulation(
                    initial,
                    honey_r(DEFAULT_BALL_RADIUS),
                    HONEY_RHO,
                    0.001,
                    &mut output_file,
                )
                .await;
            })
        }));
    }

    {
        // radius variation
        ensure_dir_exists("uppgifter/2/c/radius").await;

        let values: Vec<f64> = vec![0.001, 0.005, 0.01, 0.05, 0.1];

        tasks.extend(values.into_iter().map(|radius| {
            tokio::spawn(async move {
                let initial = FreeFallObjectSnapshot {
                    volume: std::f64::consts::PI * 4.0 * radius.powi(3) / 3.0,
                    ..*DEFAULT_BALL
                };

                let mut output_file =
                    File::create(&format!("uppgifter/2/c/radius/radius-{:.3}.csv", radius))
                        .await
                        .unwrap();
                uppgift2_run_simulation(
                    initial,
                    honey_r(radius),
                    HONEY_RHO,
                    0.001,
                    &mut output_file,
                )
                .await;
            })
        }));
    }

    for task in tasks {
        task.await.unwrap();
    }
}
