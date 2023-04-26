use fysik3_simulering::{
    spawn_timed_task, Float, FreeFallObject, FreeFallObjectSnapshot, PhysicsSystem,
};
use lazy_static::lazy_static;
use nalgebra::{vector, Vector2};
use tokio::{
    io::{AsyncWrite, AsyncWriteExt},
    join,
};

use crate::vector_len;

mod prelude {
    pub use super::{run_simulation, DEFAULT_BALL, DEFAULT_R, HONEY_RHO};
    pub use fysik3_simulering::{
        ensure_dir_exists, Float, FreeFallObject, FreeFallObjectSnapshot, PhysicsSystem,
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

const ACCELERATION_STOP_THRESHHOLD: Float = 0.001;
const DEFAULT_BALL_RADIUS: Float = 0.005;

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

fn floating_force(object: &FreeFallObjectSnapshot, g: Float, rho: Float) -> Vector2<Float> {
    vector![0.0, 1.0] * rho * g * object.volume
}

fn gravity_force(object: &FreeFallObjectSnapshot, g: Float) -> Vector2<Float> {
    vector![0.0, -1.0] * object.mass * g
}

fn dampening_force(object: &FreeFallObjectSnapshot, r: Float) -> Vector2<Float> {
    -object.velocity * r
}

pub async fn uppgift_2() {
    /*join!(
        //spawn_timed_task("2-a", del_a::uppgift_a),
        //spawn_timed_task("2-b", del_b::uppgift_b),
        //spawn_timed_task("2-c", del_c::uppgift_c)
    );*/
}

pub async fn run_simulation<W: AsyncWrite + Unpin>(
    init_snapshot: FreeFallObjectSnapshot,
    r: Float,
    rho: Float,
    dt: Float,
    output: &mut W,
) {
    let g = 9.82;

    let mut object = FreeFallObject {
        snapshot: init_snapshot,
        forces: vec![
            Box::new(move |o| gravity_force(o, g)),
            Box::new(move |o| floating_force(o, g, rho)),
            Box::new(move |o| dampening_force(o, r)),
        ],
    };

    let mut t = 0.0;

    output.write_all(b"t,v,y").await.unwrap();

    loop {
        let buf = format!(
            "{}, {}, {}",
            t, object.snapshot.velocity[1], object.snapshot.position[1]
        );
        output.write_all(buf.as_bytes()).await.unwrap();

        if vector_len(object.step_forward(dt).acceleration) < ACCELERATION_STOP_THRESHHOLD {
            break;
        }

        /*if t > 100.0 {
            break;
        }*/

        t += dt;
    }
}
