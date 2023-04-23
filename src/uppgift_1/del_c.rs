use super::{prelude::*, BALL_AIR_RESISTANCE, BALL_SNAPSHOT};

pub fn uppgift_c() {
    let default_alpha = 35.0f64.to_radians();
    let default_velocity = 40.0;

    let dt = 0.01;

    {
        // mass variation
        ensure_dir_exists("uppgifter/1/c/mass");
        let values = vec![0.1, 0.4, 1.0, 5.0, 10.0, 50.0];

        for mass in values {
            let init_snapshot = FreeFallObjectSnapshot {
                mass,
                ..BALL_SNAPSHOT
            };

            let mut output_file =
                File::create(&format!("uppgifter/1/c/mass/mass-{mass}.csv")).unwrap();
            run_simulation(init_snapshot, BALL_AIR_RESISTANCE, dt, &mut output_file);
        }
    }

    {
        // initial y variation
        ensure_dir_exists("uppgifter/1/c/initial-y");
        let values = vec![0.0, 5.0, 10.0, 25.0, 50.0, 100.0];

        for initial_y in values {
            let init_snapshot = FreeFallObjectSnapshot {
                position: vector![0.0, initial_y],
                ..BALL_SNAPSHOT
            };

            let mut output_file = File::create(&format!(
                "uppgifter/1/c/initial-y/initial-y-{initial_y}.csv"
            ))
            .unwrap();
            run_simulation(init_snapshot, BALL_AIR_RESISTANCE, dt, &mut output_file);
        }
    }

    {
        // initial velocity variation
        ensure_dir_exists("uppgifter/1/c/initial-velocity");
        let values = vec![10.0, 20.0, 40.0, 100.0, 200.0, 500.0];

        for initial_velocity in values {
            let init_snapshot = FreeFallObjectSnapshot {
                velocity: vector![default_alpha.cos(), default_alpha.sin()] * initial_velocity,
                ..BALL_SNAPSHOT
            };

            let mut output_file = File::create(&format!(
                "uppgifter/1/c/initial-velocity/initial-velocity-{initial_velocity}.csv"
            ))
            .unwrap();
            run_simulation(init_snapshot, BALL_AIR_RESISTANCE, dt, &mut output_file);
        }
    }

    {
        // angle variation
        ensure_dir_exists("uppgifter/1/c/angle");
        let values: Vec<Float> = vec![10.0, 20.0, 35.0, 45.0, 60.0, 75.0];

        for angle in values {
            let init_snapshot = FreeFallObjectSnapshot {
                velocity: vector![angle.to_radians().cos(), angle.to_radians().sin()]
                    * default_velocity,
                ..*BALL_SNAPSHOT
            };

            let mut output_file =
                File::create(&format!("uppgifter/1/c/angle/angle-{angle}.csv")).unwrap();
            run_simulation(init_snapshot, *BALL_AIR_RESISTANCE, dt, &mut output_file);
        }
    }
}
