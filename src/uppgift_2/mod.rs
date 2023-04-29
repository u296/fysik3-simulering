use fysik3_simulering::{
    data::Data, simulation::run_simulation, solver::EulerCromerSolver, spawn_timed_task,
    AppliedDynamics, Float, ForceFunction, FreeFallObjectSnapshot,
};
use lazy_static::lazy_static;
use nalgebra::vector;
use tokio::{io::AsyncWrite, join};

mod prelude {
    pub use super::{uppgift2_run_simulation, DEFAULT_BALL, DEFAULT_R, HONEY_RHO, OIL_RHO};
    pub use fysik3_simulering::{ensure_dir_exists, Float, FreeFallObject, FreeFallObjectSnapshot};
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
const DEFAULT_BALL_RADIUS: Float = 0.01;

lazy_static! {
    pub static ref DEFAULT_BALL: FreeFallObjectSnapshot<2> = FreeFallObjectSnapshot {
        mass: 0.5,
        frontal_area: DEFAULT_BALL_RADIUS.powi(2) * std::f64::consts::PI,
        volume: std::f64::consts::PI * 4.0 * DEFAULT_BALL_RADIUS.powi(3) / 3.0,
        position: vector![0.0, 0.0],
        velocity: vector![0.0, 0.0],
        angular_velocity: vector![0.0, 0.0],
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

pub async fn uppgift2_run_simulation<W: AsyncWrite + Unpin + Send>(
    init_snapshot: FreeFallObjectSnapshot<2>,
    r: Float,
    rho: Float,
    dt: Float,
    output: &mut W,
) {
    let g = 9.82;

    let forces: Vec<ForceFunction<2>> = vec![
        Box::new(move |object| vector![0.0, -1.0] * object.mass * g),
        Box::new(move |object| vector![0.0, 1.0] * rho * g * object.volume),
        Box::new(move |object| -object.velocity * r),
    ];

    struct Uppg2Data;

    impl Data<2, 3, AppliedDynamics<2>, ()> for Uppg2Data {
        fn new_datapoint(
            time: Float,
            object: &FreeFallObjectSnapshot<2>,
            _: &AppliedDynamics<2>,
            _: &(),
        ) -> [Float; 3] {
            [time, object.velocity.y, object.position.y]
        }

        fn column_names() -> [&'static str; 3] {
            ["t", "v", "y"]
        }

        fn should_end(
            _: Float,
            _: &FreeFallObjectSnapshot<2>,
            applied: &AppliedDynamics<2>,
            _: &[[Float; 3]],
            _: &(),
        ) -> bool {
            applied.acceleration.magnitude() < ACCELERATION_STOP_THRESHHOLD
        }
    }

    run_simulation::<Uppg2Data, 2, 3, _, _, _, _>(
        EulerCromerSolver::<2>::new,
        init_snapshot,
        forces,
        dt,
        (),
        output,
    )
    .await;
}
