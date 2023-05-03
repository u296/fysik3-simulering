mod prelude {
    pub use super::{uppgift3_run_simulation, DEFAULT_INIT_SNAPSHOT, DEFAULT_K};
    pub use fysik3_simulering::{
        ensure_dir_exists,
        simulation::run_simulation,
        solver::{EulerCromerSolver, EulerSolver},
        BodySnapshot,
    };
    pub use nalgebra::vector;
    pub use tokio::fs::File;
}

use fysik3_simulering::{
    data::DataLogger,
    forces::{fluid_resistance, spring_force},
    solver::{SingleObjectPhysicsSystemSolver, Step},
    spawn_timed_task, Body, Float,
};
use prelude::*;
use tokio::{io::AsyncWrite, join};

use del_a::uppgift_a;
use del_b::uppgift_b;
use del_c::uppgift_c;
use del_d::uppgift_d;
use del_e::uppgift_e;

mod del_a;
mod del_b;
mod del_c;
mod del_d;
mod del_e;

const ENERGY_THRESHHOLD: Float = 0.000000001;

pub const DEFAULT_INIT_SNAPSHOT: BodySnapshot<1> = BodySnapshot {
    mass: 1.0,
    moment_of_inertia: 0.0,
    frontal_area: 0.0,
    volume: 0.0,
    position: vector![10.0],
    velocity: vector![0.0],
    angular_velocity: vector![0.0],
};
pub const DEFAULT_K: Float = 100.0;

pub async fn uppgift_3() {
    let (a, b, c, d, e) = join!(
        spawn_timed_task("3-a", uppgift_a),
        spawn_timed_task("3-b", uppgift_b),
        spawn_timed_task("3-c", uppgift_c),
        spawn_timed_task("3-d", uppgift_d),
        spawn_timed_task("3-e", uppgift_e),
    );
    [a, b, c, d, e].into_iter().for_each(|x| x.unwrap());
}

pub async fn uppgift3_run_simulation<
    W: Unpin + AsyncWrite + Send + Sync,
    P: SingleObjectPhysicsSystemSolver<1, StepType = Step<1>>,
>(
    init_snapshot: BodySnapshot<1>,
    k: Float,
    r: Float,
    dt: Float,
    output: &mut W,
    solver_new: impl Fn(Body<1>, Float) -> P,
) {
    let solver = solver_new(
        Body {
            snapshot: init_snapshot,
            forces: vec![
                Box::new(move |o| spring_force(o, k)),
                Box::new(move |o| fluid_resistance(o, r)),
            ],
            torques: vec![],
        },
        dt,
    );

    let datalogger = Uppg3Data { output };

    run_simulation::<_, 1, 5, _, _, _>(solver, k, datalogger).await;
}

struct Uppg3Data<'a, W> {
    output: &'a mut W,
}

impl<'a, W: AsyncWrite + Send + Sync + Unpin> DataLogger<1, 5, Step<1>, Float, W>
    for Uppg3Data<'a, W>
{
    fn new_datapoint(
        &mut self,
        time: Float,
        object: &BodySnapshot<1>,
        step: &Step<1>,
        &k: &Float,
    ) -> [Float; 5] {
        let potential_energy = k * object.position.magnitude().powi(2) / 2.0;
        let kinetic_energy = object.mass * object.velocity.magnitude().powi(2) / 2.0;
        let mech_energy = potential_energy + kinetic_energy;
        [
            time,
            object.position.x,
            object.velocity.x,
            step.applied.acceleration.x,
            mech_energy,
        ]
    }

    fn column_names() -> [&'static str; 5] {
        [
            "t (s)",
            "x (m)",
            "v (m/s)",
            "a (m/s²)",
            "mekanisk energi (J)",
        ]
    }

    fn should_end(
        &mut self,
        time: Float,
        body: &BodySnapshot<1>,
        _: &Step<1>,
        _: &[[Float; 5]],
        &k: &Float,
    ) -> bool {
        let potential_energy = k * body.position.magnitude().powi(2) / 2.0;
        let kinetic_energy = body.mass * body.velocity.magnitude().powi(2) / 2.0;
        let mech_energy = potential_energy + kinetic_energy;

        time > 10.0 || (mech_energy < ENERGY_THRESHHOLD)
    }

    fn get_output(&mut self) -> &mut W {
        self.output
    }
}
