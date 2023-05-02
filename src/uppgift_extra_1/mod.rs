use fysik3_simulering::{
    data::Data,
    forces::{fluid_resistance, spring_force},
    simulation::run_simulation,
    solver::SingleObjectPhysicsSystemSolver,
    spawn_timed_task, AppliedDynamics, Body, BodySnapshot, Float,
};
use nalgebra::vector;
use prelude::*;
use tokio::join;

use del_b::uppgift_b;
use del_c::uppgift_c;

pub mod prelude {
    pub use super::{uppgift_extra_1_run_simulation, DEFAULT_INIT_SNAPSHOT, DEFAULT_K};
    pub use fysik3_simulering::{
        ensure_dir_exists,
        solver::{EulerCromerSolver, EulerSolver},
    };
    pub use tokio::{fs::File, io::AsyncWrite};
}

mod del_b;
mod del_c;

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

pub async fn uppgift_extra_1() {
    let (a, b) = join!(
        spawn_timed_task("e1-b", uppgift_b),
        spawn_timed_task("e1-c", uppgift_c)
    );
    [a, b].into_iter().for_each(Result::unwrap);
}

pub async fn uppgift_extra_1_run_simulation<
    W: Unpin + AsyncWrite + Send,
    P: SingleObjectPhysicsSystemSolver<1, Applied = AppliedDynamics<1>>,
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

    struct UppgE1Data;

    impl Data<1, 5, AppliedDynamics<1>, Float> for UppgE1Data {
        fn new_datapoint(
            time: Float,
            object: &BodySnapshot<1>,
            applied: &AppliedDynamics<1>,
            &k: &Float,
        ) -> [Float; 5] {
            let potential_energy = k * object.position.magnitude().powi(2) / 2.0;
            let kinetic_energy = object.mass * object.velocity.magnitude().powi(2) / 2.0;
            let mech_energy = potential_energy + kinetic_energy;
            [
                time,
                object.position.x,
                object.velocity.x,
                applied.acceleration.x,
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
            time: Float,
            _: &BodySnapshot<1>,
            _: &AppliedDynamics<1>,
            _: &[[Float; 5]],
            _: &Float,
        ) -> bool {
            time > 10.0
        }
    }

    run_simulation::<UppgE1Data, 1, 5, _, _, _>(solver, k, output).await;
}
