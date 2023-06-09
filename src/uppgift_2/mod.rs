use fysik3_simulering::{
    data::DataLogger,
    forces::{buoyancy, fluid_resistance, gravity},
    simulation::run_simulation,
    solver::{EulerCromerSolver, Step},
    spawn_timed_task, Body, BodySnapshot, Float,
};
use lazy_static::lazy_static;
use nalgebra::vector;
use tokio::{io::AsyncWrite, join};

mod prelude {
    pub use super::{
        honey_r, oil_r, uppgift2_run_simulation, DEFAULT_BALL, DEFAULT_BALL_RADIUS, HONEY_RHO,
        OIL_RHO,
    };
    pub use fysik3_simulering::{ensure_dir_exists, Body, BodySnapshot, Float};
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
pub const DEFAULT_BALL_RADIUS: Float = 0.01;
const IRON_DENSITY: Float = 7874.0;

lazy_static! {
    pub static ref DEFAULT_BALL: BodySnapshot<1> = {
        let volume = std::f64::consts::PI * 4.0 * DEFAULT_BALL_RADIUS.powi(3) / 3.0;
        BodySnapshot {
            mass: IRON_DENSITY * volume,
            moment_of_inertia: 0.0,
            frontal_area: 0.0,
            volume,
            position: vector![0.0],
            velocity: vector![0.0],
            angular_velocity: vector![0.0],
        }
    };
}

pub fn honey_r(radius: Float) -> Float {
    radius * 90.0
}

pub fn oil_r(radius: Float) -> Float {
    radius * 0.9
}

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

pub async fn uppgift2_run_simulation<W: AsyncWrite + Unpin + Send + Sync>(
    init_snapshot: BodySnapshot<1>,
    r: Float,
    rho: Float,
    dt: Float,
    output: &mut W,
) {
    let solver = EulerCromerSolver::new(
        Body {
            snapshot: init_snapshot,
            forces: vec![
                Box::new(gravity),
                Box::new(move |o| buoyancy(o, rho)),
                Box::new(move |o| fluid_resistance(o, r)),
            ],
            torques: vec![],
        },
        dt,
    );

    let datalogger = Uppg2Data { output };

    run_simulation::<_, 1, 4, _, _, _>(solver, r, datalogger).await;
}

struct Uppg2Data<'a, W> {
    output: &'a mut W,
}

impl<'a, W: AsyncWrite + Send + Sync + Unpin> DataLogger<1, 4, Step<1>, Float, W>
    for Uppg2Data<'a, W>
{
    fn new_datapoint(
        &mut self,
        time: Float,
        object: &BodySnapshot<1>,
        step: &Step<1>,
        &r: &Float,
    ) -> [Float; 4] {
        [
            time,
            object.velocity.x,
            object.position.x,
            step.applied.force.magnitude() / r,
        ]
    }

    fn column_names() -> [&'static str; 4] {
        ["t (s)", "v (m/s)", "y (m)", "F / r"]
    }

    fn should_end(
        &mut self,
        _: Float,
        _: &BodySnapshot<1>,
        step: &Step<1>,
        _: &[[Float; 4]],
        _: &Float,
    ) -> bool {
        step.applied.acceleration.magnitude() < ACCELERATION_STOP_THRESHHOLD
    }

    fn get_output(&mut self) -> &mut W {
        self.output
    }
}
