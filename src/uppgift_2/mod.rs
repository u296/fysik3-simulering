use fysik3_simulering::{
    euler_cromer::EulerCromerSolver, spawn_timed_task, Float, FreeFallObject,
    FreeFallObjectSnapshot, PhysicsSystemSolver,
};
use lazy_static::lazy_static;
use nalgebra::vector;
use tokio::{
    io::{AsyncWrite, AsyncWriteExt, BufWriter},
    join,
};

use crate::vector_len;

mod prelude {
    pub use super::{run_simulation, DEFAULT_BALL, DEFAULT_R, HONEY_RHO, OIL_RHO};
    pub use fysik3_simulering::{
        ensure_dir_exists, Float, FreeFallObject, FreeFallObjectSnapshot, PhysicsSystemSolver,
    };
    pub use nalgebra::{vector, Vector2};
    pub use std::{io::Write, path::Path, time::Instant};
    pub use tokio::{fs::File, io::AsyncWrite};
}

mod del_a;
mod del_b;
mod del_c;
mod del_d;
mod del_e;

const NUM_DATAPOINTS: usize = 2000;
const ACCELERATION_STOP_THRESHHOLD: Float = 0.001;
const DEFAULT_BALL_RADIUS: Float = 0.01;

lazy_static! {
    pub static ref DEFAULT_BALL: FreeFallObjectSnapshot = FreeFallObjectSnapshot {
        mass: 0.5,
        charge: 0.0,
        frontal_area: DEFAULT_BALL_RADIUS.powi(2) * std::f64::consts::PI,
        volume: std::f64::consts::PI * 4.0 * DEFAULT_BALL_RADIUS.powi(3) / 3.0,
        position: vector![0.0, 0.0],
        velocity: vector![0.0, 0.0],
    };
}

pub const DEFAULT_R: Float = DEFAULT_BALL_RADIUS * 90.0;
pub const HONEY_RHO: Float = 1420.0;
pub const OIL_RHO: Float = 918.0;

pub async fn uppgift_2() {
    let (a, b, c, d, e) = join!(
        spawn_timed_task("2-a", del_a::uppgift_a),
        spawn_timed_task("2-b", del_b::uppgift_b),
        spawn_timed_task("2-c", del_c::uppgift_c),
        spawn_timed_task("2-d", del_d::uppgift_d),
        spawn_timed_task("2-e", del_e::uppgift_e),
    );
    [a, b, c, d, e].into_iter().for_each(|x| x.unwrap());
}

pub async fn run_simulation<W: AsyncWrite + Unpin>(
    init_snapshot: FreeFallObjectSnapshot,
    r: Float,
    rho: Float,
    dt: Float,
    output: &mut W,
) {
    let g = 9.82;

    let mut solver = EulerCromerSolver::new(
        FreeFallObject {
            snapshot: init_snapshot,
            forces: vec![
                Box::new(move |object| vector![0.0, -1.0] * object.mass * g),
                Box::new(move |object| vector![0.0, 1.0] * rho * g * object.volume),
                Box::new(move |object| -object.velocity * r),
            ],
        },
        dt,
    );

    let mut t = 0.0;

    let mut datapoints = Vec::new();

    loop {
        datapoints.push([
            t,
            solver.object.snapshot.velocity[1],
            solver.object.snapshot.position[1],
        ]);

        if vector_len(solver.step_forward().acceleration) < ACCELERATION_STOP_THRESHHOLD {
            break;
        }

        /*if t > 100.0 {
            break;
        }*/

        t += dt;
    }

    let mut output_writer = BufWriter::new(output);

    output_writer.write_all(b"t,v,y\n").await.unwrap();

    let num_datapoints = datapoints.len();

    if num_datapoints > NUM_DATAPOINTS {
        let mut index = 0.0;

        let step_size = num_datapoints as f32 / NUM_DATAPOINTS as f32;

        while let Some(datapoint) = datapoints.get(index as usize) {
            let buf = format!("{}, {}, {}\n", datapoint[0], datapoint[1], datapoint[2]);
            output_writer.write_all(buf.as_bytes()).await.unwrap();
            index += step_size;
        }
    } else {
        for datapoint in datapoints {
            let buf = format!("{}, {}, {}\n", datapoint[0], datapoint[1], datapoint[2]);
            output_writer.write_all(buf.as_bytes()).await.unwrap();
        }
    }

    output_writer.flush().await.unwrap();
}
