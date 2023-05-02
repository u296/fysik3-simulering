use fysik3_simulering::{
    data::DataLogger,
    ensure_dir_exists,
    forces::{air_resistance, gravity, magnus_effect, AirResistanceParameters},
    simulation::run_simulation,
    solver::{EulerCromerSolver, Step},
    torques::air_resistance_torque,
    AppliedDeltas, AppliedDynamics, Body, BodySnapshot, Float,
};
use nalgebra::vector;
use tokio::{fs::File, io::AsyncWrite};

pub const DEFAULT_RADIUS: Float = 0.11;

/* fotboll
   C_D = 0.25
   radius: 11 cm
   mass 450 g


*/

pub async fn uppgift_extra_2() {
    let omega = 2.0 * std::f64::consts::PI;
    let component = std::f64::consts::FRAC_1_SQRT_2;

    let init = BodySnapshot {
        mass: 0.45,
        moment_of_inertia: (2.0 / 5.0) * 0.4 * DEFAULT_RADIUS.powi(2),
        frontal_area: DEFAULT_RADIUS.powi(2) * std::f64::consts::PI,
        volume: 0.0,
        position: vector![0.0, 0.0, 0.0],
        velocity: vector![
            35.0f64.to_radians().cos() * 40.0,
            35.0f64.to_radians().sin() * 40.0,
            0.0
        ],
        angular_velocity: vector![0.0, component, component] * omega,
    };

    let air_resistance_params = AirResistanceParameters {
        rho: 1.2,
        c_d: 0.25,
    };

    ensure_dir_exists("uppgifter/extra_2").await;

    let mut output = File::create(&format!("uppgifter/extra_2/result.csv"))
        .await
        .unwrap();

    uppgift_extra_2_run_simulation(
        init,
        air_resistance_params,
        DEFAULT_RADIUS,
        0.001,
        &mut output,
    )
    .await;
}

pub async fn uppgift_extra_2_run_simulation<W: AsyncWrite + Unpin + Send>(
    initial_snapshot: BodySnapshot<3>,
    air_resistance_params: AirResistanceParameters,
    radius: Float,
    dt: Float,
    output: &mut W,
) {
    let solver = EulerCromerSolver::new(
        Body {
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

    struct UppgE2Data {
        s: Float,
    };

    impl DataLogger<3, 11, Step<AppliedDynamics<3>, AppliedDeltas<3>>, (), File> for UppgE2Data {
        fn new_datapoint(
            &mut self,
            time: Float,
            object: &BodySnapshot<3>,
            step: &Step<AppliedDynamics<3>, AppliedDeltas<3>>,
            _: &(),
        ) -> [Float; 11] {
            let kinetic = 0.5 * object.mass * object.velocity.magnitude_squared();
            let potential = object.mass * object.position[1] * 9.82;

            let energy = kinetic + potential;

            [
                time,
                object.position.x,
                object.position.y,
                object.position.z,
                self.s,
                energy,
                object.velocity.x,
                object.velocity.y,
                object.velocity.z,
                object.angular_velocity.magnitude(),
                step.applied.angular_acceleration.magnitude(),
            ]
        }

        fn column_names() -> [&'static str; 11] {
            [
                "t (s)",
                "x (m)",
                "y (m)",
                "z (m)",
                "s (m)",
                "translationell mekanisk energi (J)",
                "vx (m/s)",
                "vy (m/s)",
                "vz (m/s)",
                "omega (rad/s)",
                "tau (Nm)",
            ]
        }

        fn should_end(
            time: Float,
            object: &BodySnapshot<3>,
            _: &Step<AppliedDynamics<3>, AppliedDeltas<3>>,
            _: &[[Float; 11]],
            _: &(),
        ) -> bool {
            object.position.y < 0.0 || time > 10.0
        }
    }

    let datalogger = UppgE2Data { s: 0.0 };

    run_simulation::<UppgE2Data, 3, 11, _, _, _>(solver, (), datalogger).await;
}
