use fysik3_simulering::{
    data::Data, ensure_dir_exists, simulation::run_simulation, solver::EulerCromerSolver,
    AppliedDynamics, Float, ForceFunction, FreeFallObjectSnapshot,
};
use nalgebra::vector;
use tokio::{fs::File, io::AsyncWrite};

use crate::uppgift_1::AirResistanceParameters;

pub async fn uppgift_extra_2() {
    let radius: Float = 0.1;

    let init = FreeFallObjectSnapshot {
        mass: 0.4,
        frontal_area: radius.powi(2) * std::f64::consts::PI,
        volume: 0.0,
        position: vector![0.0, 0.0, 0.0],
        velocity: vector![
            35.0f64.to_radians().cos() * 40.0,
            35.0f64.to_radians().sin() * 40.0,
            0.0
        ],
        angular_velocity: vector![0.0, 5.0, 0.0],
    };

    let air_resistance_params = AirResistanceParameters {
        rho: 1.2,
        c_d: 0.47,
    };

    ensure_dir_exists("uppgifter/extra_2").await;

    let mut output = File::create("uppgifter/extra_2/results.csv").await.unwrap();
    uppgift_extra_2_run_simulation(init, air_resistance_params, radius, 0.01, &mut output).await;
}

pub async fn uppgift_extra_2_run_simulation<W: AsyncWrite + Unpin + Send>(
    initial_snapshot: FreeFallObjectSnapshot<3>,
    air_resistance_params: AirResistanceParameters,
    radius: Float,
    dt: Float,
    output: &mut W,
) {
    let forces: Vec<ForceFunction<3>> = vec![
        Box::new(move |o| o.mass * 9.82 * vector![0.0, -1.0, 0.0]), // gravity
        Box::new(move |o| {
            // air resistance
            0.5 * air_resistance_params.c_d
                * air_resistance_params.rho
                * o.frontal_area
                * -1.0
                * o.velocity
                * o.velocity.magnitude()
        }),
        Box::new(move |o| {
            // magnus effect
            let f = 2.0
                * air_resistance_params.rho
                * o.velocity.magnitude()
                * o.angular_velocity.magnitude()
                * radius
                * o.velocity.cross(&o.angular_velocity).normalize();
            println!("{}", f.magnitude());
            f
        }),
    ];

    struct UppgE2Data;

    impl Data<3, 5, AppliedDynamics<3>, ()> for UppgE2Data {
        fn new_datapoint(
            time: Float,
            object: &FreeFallObjectSnapshot<3>,
            _: &AppliedDynamics<3>,
            _: &(),
        ) -> [Float; 5] {
            let kinetic = 0.5 * object.mass * object.velocity.magnitude_squared();
            let potential = object.mass * object.position[1] * 9.82;

            let energy = kinetic + potential;

            [
                time,
                object.position.x,
                object.position.y,
                object.position.z,
                energy,
            ]
        }

        fn column_names() -> [&'static str; 5] {
            ["t", "x", "y", "z", "mechanical energy"]
        }

        fn should_end(
            _: Float,
            object: &FreeFallObjectSnapshot<3>,
            _: &AppliedDynamics<3>,
            _: &[[Float; 5]],
            _: &(),
        ) -> bool {
            object.position.y < 0.0
        }
    }

    run_simulation::<UppgE2Data, 3, 5, _, _, _, _>(
        EulerCromerSolver::<3>::new,
        initial_snapshot,
        forces,
        dt,
        (),
        output,
    )
    .await;
}
