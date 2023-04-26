use super::prelude::*;

pub async fn uppgift_d() {
    let init_snapshot = FreeFallObjectSnapshot {
        mass: 1.0,
        charge: 0.0,
        frontal_area: 0.0,
        volume: 0.0,
        position: vector![10.0, 0.0],
        velocity: vector![0.0, 0.0],
    };

    ensure_dir_exists("uppgifter/3/d").await;

    {
        let mut output = File::create("uppgifter/3/d/svag.csv").await.unwrap();
        run_simulation(init_snapshot, 100.0, 1.0, 0.01, &mut output).await;
    }

    // theoretical critical: r = 20.0 = 2m * sqrt(k/m)

    {
        let mut output = File::create("uppgifter/3/d/stark.csv").await.unwrap();
        run_simulation(init_snapshot, 100.0, 50.0, 0.01, &mut output).await;
    }
}
