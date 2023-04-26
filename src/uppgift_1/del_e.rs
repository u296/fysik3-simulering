use super::{prelude::*, AIRCRAFT_RESISTANCE, AIRCRAFT_SNAPSHOT};

pub async fn uppgift_e() {
    ensure_dir_exists("uppgifter/1/e").await;

    let vals = vec![0.001, 0.005, 0.01, 0.05, 0.1];

    for dt in vals {
        let mut output_file = File::create(&format!("uppgifter/1/e/dt-{dt}.csv"))
            .await
            .unwrap();
        run_simulation(
            *AIRCRAFT_SNAPSHOT,
            *AIRCRAFT_RESISTANCE,
            dt,
            &mut output_file,
        )
        .await
    }
}
