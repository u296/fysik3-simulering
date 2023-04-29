use fysik3_simulering::{
    data::Data,
    ensure_dir_exists,
    forces::{air_resistance, gravity, magnus_effect, AirResistanceParameters},
    simulation::run_simulation,
    solver::EulerCromerSolver,
    torques::air_resistance_torque,
    AppliedDynamics, Float, FreeFallObject, FreeFallObjectSnapshot,
};
use nalgebra::vector;
use tokio::{fs::File, io::AsyncWrite};

pub const DEFAULT_RADIUS: Float = 0.1;

pub async fn uppgift_extra_2() {
    let init = FreeFallObjectSnapshot {
        mass: 0.4,
        moment_of_inertia: (2.0 / 5.0) * 0.4 * DEFAULT_RADIUS.powi(2),
        frontal_area: DEFAULT_RADIUS.powi(2) * std::f64::consts::PI,
        volume: 0.0,
        position: vector![0.0, 0.0, 0.0],
        velocity: vector![
            35.0f64.to_radians().cos() * 40.0,
            35.0f64.to_radians().sin() * 40.0,
            0.0
        ],
        angular_velocity: vector![0.0, 0.0, -5.0],
    };

    let air_resistance_params = AirResistanceParameters {
        rho: 1.2,
        c_d: 0.47,
    };

    ensure_dir_exists("uppgifter/extra_2/velocity").await;

    let vals = vec![5.0, 10.0, 15.0, 20.0, 30.0];

    let tasks: Vec<_> = vals
        .into_iter()
        .map(|v| {
            tokio::spawn(async move {
                let mut output = File::create(&format!("uppgifter/extra_2/velocity/{v}.csv"))
                    .await
                    .unwrap();

                let mut init = init;

                init.velocity.set_magnitude(v);

                uppgift_extra_2_run_simulation(
                    init,
                    air_resistance_params,
                    DEFAULT_RADIUS,
                    0.01,
                    &mut output,
                )
                .await;
            })
        })
        .collect();

    for task in tasks {
        task.await.unwrap();
    }
}

pub async fn uppgift_extra_2_run_simulation<W: AsyncWrite + Unpin + Send>(
    initial_snapshot: FreeFallObjectSnapshot<3>,
    air_resistance_params: AirResistanceParameters,
    radius: Float,
    dt: Float,
    output: &mut W,
) {
    let solver = EulerCromerSolver::new(
        FreeFallObject {
            snapshot: initial_snapshot,
            forces: vec![
                Box::new(gravity), // gravity
                Box::new(move |o| air_resistance(o, air_resistance_params)),
                Box::new(move |o| magnus_effect(o, radius, air_resistance_params.rho)),
            ],
            torques: vec![Box::new(move |o| {
                air_resistance_torque(o, radius, air_resistance_params)
            })],
        },
        dt,
    );

    struct UppgE2Data;

    impl Data<3, 9, AppliedDynamics<3>, ()> for UppgE2Data {
        fn new_datapoint(
            time: Float,
            object: &FreeFallObjectSnapshot<3>,
            applied: &AppliedDynamics<3>,
            _: &(),
        ) -> [Float; 9] {
            let kinetic = 0.5 * object.mass * object.velocity.magnitude_squared();
            let potential = object.mass * object.position[1] * 9.82;

            let energy = kinetic + potential;

            [
                time,
                object.position.x,
                object.position.y,
                object.position.z,
                energy,
                object.velocity.x,
                object.velocity.z,
                object.angular_velocity.magnitude(),
                applied.angular_acceleration.magnitude(),
            ]
        }

        fn column_names() -> [&'static str; 9] {
            [
                "t",
                "x",
                "y",
                "z",
                "translational mechanical energy",
                "vx",
                "vz",
                "omega",
                "tau",
            ]
        }

        fn should_end(
            time: Float,
            object: &FreeFallObjectSnapshot<3>,
            _: &AppliedDynamics<3>,
            _: &[[Float; 9]],
            _: &(),
        ) -> bool {
            object.position.y < 0.0 || time > 10.0
        }
    }

    run_simulation::<UppgE2Data, 3, 9, _, _, _>(solver, (), output).await;
}
