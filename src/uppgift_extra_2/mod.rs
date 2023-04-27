use fysik3_simulering::{
    euler_cromer::EulerCromerSolver, Float, FreeFallObject, FreeFallObjectSnapshot,
};
use nalgebra::vector;
use tokio::io::AsyncWrite;

use crate::uppgift_1::AirResistanceParameters;

pub async fn run_simulation<W: AsyncWrite + Unpin>(
    initial_snapshot: FreeFallObjectSnapshot<3>,
    air_resistance_params: AirResistanceParameters,
    radius: Float,
    dt: Float,
    output: &mut W,
) {
    const G: Float = 9.82;

    let solver = EulerCromerSolver::new(
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
                    2.0 * air_resistance_params.rho
                        * o.velocity.magnitude()
                        * o.angular_velocity.magnitude()
                        * radius
                        * cross
                }),
            ],
        },
        dt,
    );
}
