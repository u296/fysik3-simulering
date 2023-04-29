use fysik3_simulering::{
    euler_cromer::EulerCromerSolver, run_simulation, AppliedDynamics, Data, Float, ForceFunction,
    FreeFallObjectSnapshot,
};
use lazy_static::lazy_static;
use nalgebra::vector;
use tokio::join;

mod prelude {
    pub use super::{uppgift1_run_simulation, AirResistanceParameters};
    pub use fysik3_simulering::{
        ensure_dir_exists, spawn_timed_task, Float, FreeFallObject, FreeFallObjectSnapshot,
        PhysicsSystemSolver,
    };
    pub use nalgebra::{vector, Vector2};
    pub use std::{io::Write, path::Path};
    pub use tokio::{fs::File, io::AsyncWrite, task};
}
use prelude::*;

mod del_b;
mod del_c;
mod del_d;
mod del_e;
mod del_f;
mod del_g;

lazy_static! {
    static ref BALL_SNAPSHOT: FreeFallObjectSnapshot<2> = FreeFallObjectSnapshot {
        mass: 0.4,
        frontal_area: 0.01 * std::f64::consts::PI,
        volume: 0.0,
        position: vector![0.0, 0.0],
        velocity: vector![
            35.0f64.to_radians().cos() * 40.0,
            35.0f64.to_radians().sin() * 40.0
        ],
        angular_velocity: vector![0.0,0.0],
    };


    static ref BALL_AIR_RESISTANCE: AirResistanceParameters = AirResistanceParameters {
        c_d: 0.47,
        rho: 1.2,
    };

/*
Simulering:

En Boeing 777-200LR har nyss lyft och flyger 10 m över marken.
Båda motorer slutar plötsligt att fungera omedelbart

Chatgpt säger att följande källor:
    * "Assessment of Fuel Consumption, Emissions, and Noise of Boeing 777 Aircraft for a Range of Configurations" (NASA/TM-2013-217245)
    * "Federal Aviation Administration's Type Certificate Data Sheet for the Boeing 777-200"
    * Boeings webbsida

anger följande information om flygplanet:
    * dragkoefficient: 0.023-0.027
    * frontalarea: 242-249 m²
    * maximal vikt: 347_450 kg
    * anfallsvinkel vid lyft: 10-15 grader
    * lyfthastighet: 130-165 knop
 */

    static ref AIRCRAFT_SNAPSHOT: FreeFallObjectSnapshot<2> = {
        let knots_to_mps = 0.51444;
        FreeFallObjectSnapshot {
            mass: 347450.0,
            frontal_area: 245.5,
            volume: 0.0,
            position: vector![0.0, 10.0],
            velocity: vector![
                12.5f64.to_radians().cos() * 165.0 * knots_to_mps,
                12.5f64.to_radians().sin() * 165.0 * knots_to_mps
            ],
            angular_velocity: vector![0.0, 0.0],
        }
    };

    static ref AIRCRAFT_RESISTANCE: AirResistanceParameters = AirResistanceParameters {
        c_d: 0.025,
        rho: 1.2,
    };
}

pub async fn uppgift_1() {
    let (b, c, d, e, f, g) = join!(
        spawn_timed_task("1-b", del_b::uppgift_b),
        spawn_timed_task("1-c", del_c::uppgift_c),
        spawn_timed_task("1-d", del_d::uppgift_d),
        spawn_timed_task("1-e", del_e::uppgift_e),
        spawn_timed_task("1-f", del_f::uppgift_f),
        spawn_timed_task("1-g", del_g::uppgift_g)
    );

    [b, c, d, e, f, g].into_iter().for_each(|x| x.unwrap());
}

#[derive(Clone, Copy)]
pub struct AirResistanceParameters {
    pub c_d: Float,
    pub rho: Float,
}

pub async fn uppgift1_run_simulation<W: AsyncWrite + Unpin>(
    initial_snapshot: FreeFallObjectSnapshot<2>,
    air_resistance_params: AirResistanceParameters,
    dt: Float,
    output: &mut W,
) {
    const G: Float = 9.82;

    let forces: Vec<ForceFunction<2>> = vec![
        Box::new(move |o| o.mass * G * vector![0.0, -1.0]),
        Box::new(move |o| {
            0.5 * air_resistance_params.c_d
                * air_resistance_params.rho
                * o.frontal_area
                * -1.0
                * o.velocity
                * o.velocity.magnitude()
        }),
    ];

    struct Uppg1Data;

    impl Data<2, 3, AppliedDynamics<2>, ()> for Uppg1Data {
        fn new_datapoint(
            time: Float,
            object: &FreeFallObjectSnapshot<2>,
            _: &AppliedDynamics<2>,
            _: &(),
        ) -> [Float; 3] {
            [time, object.position[0], object.position[1]]
        }

        fn column_names() -> [&'static str; 3] {
            ["t", "x", "y"]
        }

        fn should_end(
            _: Float,
            object: &FreeFallObjectSnapshot<2>,
            _: &AppliedDynamics<2>,
            _: &[[Float; 3]],
            _: &(),
        ) -> bool {
            object.position.y < 0.0
        }
    }

    run_simulation::<Uppg1Data, 2, 3, _, _, _, _>(
        EulerCromerSolver::<2>::new,
        initial_snapshot,
        forces,
        dt,
        (),
        output,
    )
    .await;
}
