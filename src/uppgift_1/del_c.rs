use super::{prelude::*, BALL_AIR_RESISTANCE, BALL_SNAPSHOT};

pub async fn uppgift_c() {
    let default_alpha = 35.0f64.to_radians();
    let default_velocity = 40.0;

    let dt = 0.001;

    {
        // mass variation
        ensure_dir_exists("uppgifter/1/c/mass").await;
        let values = vec![0.1, 0.4, 1.0, 5.0, 10.0, 50.0];

        for mass in values {
            let init_snapshot = BodySnapshot {
                mass,
                ..*BALL_SNAPSHOT
            };

            let mut output_file =
                File::create(&format!("uppgifter/1/c/mass/mass-{:0>4.1}.csv", mass))
                    .await
                    .unwrap();
            uppgift1_run_simulation(init_snapshot, *BALL_AIR_RESISTANCE, dt, &mut output_file)
                .await;
        }
    }

    {
        // initial y variation
        ensure_dir_exists("uppgifter/1/c/initial-y").await;
        let values = vec![0.0, 5.0, 10.0, 25.0, 50.0, 100.0];

        for initial_y in values {
            let init_snapshot = BodySnapshot {
                position: vector![0.0, initial_y],
                ..*BALL_SNAPSHOT
            };

            let mut output_file = File::create(&format!(
                "uppgifter/1/c/initial-y/initial-y-{:0>3}.csv",
                initial_y
            ))
            .await
            .unwrap();
            uppgift1_run_simulation(init_snapshot, *BALL_AIR_RESISTANCE, dt, &mut output_file)
                .await;
        }
    }

    {
        // initial velocity variation
        ensure_dir_exists("uppgifter/1/c/initial-velocity").await;
        let values = vec![10.0, 20.0, 40.0, 100.0, 200.0, 500.0];

        for initial_velocity in values {
            let init_snapshot = BodySnapshot {
                velocity: vector![default_alpha.cos(), default_alpha.sin()] * initial_velocity,
                ..*BALL_SNAPSHOT
            };

            let mut output_file = File::create(&format!(
                "uppgifter/1/c/initial-velocity/initial-velocity-{:0>3}.csv",
                initial_velocity
            ))
            .await
            .unwrap();
            uppgift1_run_simulation(init_snapshot, *BALL_AIR_RESISTANCE, dt, &mut output_file)
                .await;
        }
    }

    {
        // angle variation
        ensure_dir_exists("uppgifter/1/c/angle").await;
        let values: Vec<Float> = vec![10.0, 20.0, 35.0, 45.0, 60.0, 75.0];

        for angle in values {
            let init_snapshot = BodySnapshot {
                velocity: vector![angle.to_radians().cos(), angle.to_radians().sin()]
                    * default_velocity,
                ..*BALL_SNAPSHOT
            };

            let mut output_file = File::create(&format!("uppgifter/1/c/angle/angle-{angle}.csv"))
                .await
                .unwrap();
            uppgift1_run_simulation(init_snapshot, *BALL_AIR_RESISTANCE, dt, &mut output_file)
                .await;
        }
    }
}
