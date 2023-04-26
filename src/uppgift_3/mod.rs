mod prelude {
    pub use super::run_simulation;
    pub use fysik3_simulering::{
        ensure_dir_exists, Float, FreeFallObject, FreeFallObjectSnapshot, PhysicsSystem,
    };
    pub use nalgebra::{vector, Vector2};
    pub use std::{fs::File, io::Write, path::Path, time::Instant};
}

use prelude::*;

pub fn run_simulation(
    init_snapshot: FreeFallObjectSnapshot,
    k: Float,
    dt: Float,
    output: &mut impl Write,
) {
}
