mod prelude {
    pub use super::{uppgift3_run_simulation, DEFAULT_INIT_SNAPSHOT, DEFAULT_K};
    pub use fysik3_simulering::{
        ensure_dir_exists, euler::EulerSolver, euler_cromer::EulerCromerSolver,
        FreeFallObjectSnapshot, PhysicsSystemSolver,
    };
    pub use nalgebra::vector;
    pub use tokio::fs::File;
}

use fysik3_simulering::{
    run_simulation, spawn_timed_task, AppliedDynamics, Data, Float, ForceFunction, FreeFallObject,
    SingleObjectPhysicsSystemSolver,
};
use prelude::*;
use tokio::{io::AsyncWrite, join};

use del_a::uppgift_a;
use del_c::uppgift_c;
use del_d::uppgift_d;
use del_e::uppgift_e;
use uppgift_extra_b::uppgift_extra_b;
use uppgift_extra_c::uppgift_extra_c;

mod del_a;
mod del_c;
mod del_d;
mod del_e;
mod uppgift_extra_b;
mod uppgift_extra_c;

pub const DEFAULT_INIT_SNAPSHOT: FreeFallObjectSnapshot<2> = FreeFallObjectSnapshot {
    mass: 1.0,
    frontal_area: 0.0,
    volume: 0.0,
    position: vector![10.0, 0.0],
    velocity: vector![0.0, 0.0],
    angular_velocity: vector![0.0, 0.0],
};
pub const DEFAULT_K: Float = 100.0;

pub async fn uppgift_3() {
    let (a, c, d, e, extra_b, extra_c) = join!(
        spawn_timed_task("3-a", uppgift_a),
        spawn_timed_task("3-c", uppgift_c),
        spawn_timed_task("3-d", uppgift_d),
        spawn_timed_task("3-e", uppgift_e),
        spawn_timed_task("3-extra-b", uppgift_extra_b),
        spawn_timed_task("3-extra-c", uppgift_extra_c),
    );
    [a, c, d, e, extra_b, extra_c]
        .into_iter()
        .for_each(|x| x.unwrap());
}

pub async fn uppgift3_run_simulation<
    W: Unpin + AsyncWrite,
    P: SingleObjectPhysicsSystemSolver<2, Applied = AppliedDynamics<2>>,
>(
    init_snapshot: FreeFallObjectSnapshot<2>,
    k: Float,
    r: Float,
    dt: Float,
    output: &mut W,
    solver_new: impl Fn(FreeFallObject<2>, Float) -> P,
) {
    let forces: Vec<ForceFunction<2>> = vec![
        Box::new(move |o| o.position * -k),
        Box::new(move |o| o.velocity * -r),
    ];

    struct Uppg3Data;

    impl Data<2, 5, AppliedDynamics<2>, Float> for Uppg3Data {
        fn new_datapoint(
            time: Float,
            object: &FreeFallObjectSnapshot<2>,
            applied: &AppliedDynamics<2>,
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
            ["t", "x", "v", "a", "E mech"]
        }

        fn should_end(
            time: Float,
            _: &FreeFallObjectSnapshot<2>,
            _: &AppliedDynamics<2>,
            _: &[[Float; 5]],
            _: &Float,
        ) -> bool {
            time > 10.0
        }
    }

    run_simulation::<Uppg3Data, 2, 5, _, _, _, _>(solver_new, init_snapshot, forces, dt, k, output)
        .await;
}
