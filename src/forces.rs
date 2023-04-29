use nalgebra::SVector;

use crate::{Float, FreeFallObjectSnapshot};

const G: Float = 9.82;

fn down<const D: usize>() -> SVector<Float, D> {
    let mut x: [Float; D] = [0.0; D];
    if D >= 2 {
        x[1] = -1.0;
    }
    x.into()
}

fn up<const D: usize>() -> SVector<Float, D> {
    let mut x: [Float; D] = [0.0; D];
    if D >= 2 {
        x[1] = 1.0;
    }
    x.into()
}

pub fn gravity<const D: usize>(o: &FreeFallObjectSnapshot<D>) -> SVector<Float, D> {
    o.mass * G * down()
}

#[derive(Clone, Copy)]
pub struct AirResistanceParameters {
    pub c_d: Float,
    pub rho: Float,
}

pub fn air_resistance<const D: usize>(
    o: &FreeFallObjectSnapshot<D>,
    air_resistance_params: AirResistanceParameters,
) -> SVector<Float, D> {
    0.5 * air_resistance_params.c_d
        * air_resistance_params.rho
        * o.frontal_area
        * -1.0
        * o.velocity
        * o.velocity.magnitude()
}

pub fn buoyancy<const D: usize>(o: &FreeFallObjectSnapshot<D>, rho: Float) -> SVector<Float, D> {
    rho * G * o.volume * up()
}

pub fn fluid_resistance<const D: usize>(
    o: &FreeFallObjectSnapshot<D>,
    r: Float,
) -> SVector<Float, D> {
    -r * o.velocity
}

pub fn spring_force<const D: usize>(o: &FreeFallObjectSnapshot<D>, k: Float) -> SVector<Float, D> {
    -k * o.position
}

pub fn magnus_effect<const D: usize>(
    o: &FreeFallObjectSnapshot<D>,
    radius: Float,
    rho: Float,
) -> SVector<Float, D> {
    2.0 * rho
        * o.velocity.magnitude()
        * o.angular_velocity.magnitude()
        * radius
        * o.velocity.cross(&o.angular_velocity).normalize()
}
