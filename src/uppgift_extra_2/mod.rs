use fysik3_simulering::{
    ensure_dir_exists, euler_cromer::EulerCromerSolver, write_datapoint, Float, FreeFallObject,
    FreeFallObjectSnapshot, PhysicsSystemSolver, SingleObjectPhysicsSystemSolver,
};
use nalgebra::vector;
use tokio::{
    fs::File,
    io::{AsyncWrite, AsyncWriteExt, BufWriter},
};

use crate::uppgift_1::AirResistanceParameters;

const NUM_DATAPOINTS: usize = 2000;

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
    run_simulation(init, air_resistance_params, radius, 0.01, &mut output).await;
}

pub async fn run_simulation<W: AsyncWrite + Unpin>(
    initial_snapshot: FreeFallObjectSnapshot<3>,
    air_resistance_params: AirResistanceParameters,
    radius: Float,
    dt: Float,
    output: &mut W,
) {
    const G: Float = 9.82;

    let mut solver = EulerCromerSolver::new(
        FreeFallObject {
            snapshot: initial_snapshot,
            forces: vec![
                Box::new(move |o| o.mass * G * vector![0.0, -1.0, 0.0]), // gravity
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
            ],
        },
        dt,
    );

    let mut t = 0.0;
    let mut datapoints = Vec::new();

    loop {
        let snapshot = &solver.get_object().snapshot;
        let kinetic = 0.5 * snapshot.mass * snapshot.velocity.magnitude_squared();
        let potential = snapshot.mass * snapshot.position[1] * 9.82;

        let energy = kinetic + potential;

        datapoints.push([
            t,
            solver.get_object().snapshot.position[0],
            solver.get_object().snapshot.position[1],
            solver.get_object().snapshot.position[2],
            energy,
        ]);

        solver.step_forward();
        t += dt;

        if solver.object.snapshot.position.y < 0.0 {
            break;
        }
    }

    let mut output_writer = BufWriter::new(output);

    output_writer
        .write_all(b"t,x,y,z,mechanical energy\n")
        .await
        .unwrap();

    if datapoints.len() > NUM_DATAPOINTS {
        let mut index = 0.0;

        let step_size = datapoints.len() as f32 / NUM_DATAPOINTS as f32;

        while let Some(datapoint) = datapoints.get(index as usize) {
            write_datapoint(&mut output_writer, *datapoint).await;
            index += step_size;
        }
    } else {
        for datapoint in datapoints {
            write_datapoint(&mut output_writer, datapoint).await;
        }
    }

    output_writer.flush().await.unwrap()
}
