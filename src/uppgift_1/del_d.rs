use super::{prelude::*, AIRCRAFT_RESISTANCE, AIRCRAFT_SNAPSHOT};

pub async fn uppgift_d() {
    ensure_dir_exists("uppgifter/1/d").await;
    let mut output_file = match File::create("uppgifter/1/d/result.csv").await {
        Ok(s) => s,
        Err(e) => {
            panic!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA: {e}");
        }
    };

    run_simulation(
        *AIRCRAFT_SNAPSHOT,
        *AIRCRAFT_RESISTANCE,
        0.01,
        &mut output_file,
    )
    .await;
}
