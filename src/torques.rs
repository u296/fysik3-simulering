use nalgebra::SVector;

use crate::{forces::AirResistanceParameters, BodySnapshot, Float};

pub fn air_resistance_torque<const D: usize>(
    o: &BodySnapshot<D>,
    r: Float,
    air_resistance_params: AirResistanceParameters,
) -> SVector<Float, D> {
    /* We can't really use the air resistance formula since there is
    no area that is traveling towards the air

    we can assume that the torque should be proportional to the area
    of the sphere (thus r²), the drag coefficient, as well as the speed
    of the surface (omega * r) which means


    unit analysis gives C * omega^2 * r^5 * rho

    since in the other equation, C = 0.5 * C_D * pi we can assume the same here


     */

    -0.5 * air_resistance_params.rho
        * air_resistance_params.c_d
        * std::f64::consts::PI
        * r.powi(5)
        * o.angular_velocity.magnitude()
        * o.angular_velocity
}
