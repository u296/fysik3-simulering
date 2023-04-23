use std::fs;
use std::io::{ErrorKind, Write};
use std::path::Path;

use fysik3_simulering::{Float, FreeFallObject, FreeFallObjectSnapshot, PhysicsSystem};
use lazy_static::lazy_static;
use nalgebra::{vector, Vector2};

use crate::vector_len;

mod prelude {
    pub use super::{run_simulation, AirResistanceParameters};
    pub use fysik3_simulering::{
        ensure_dir_exists, Float, FreeFallObject, FreeFallObjectSnapshot, PhysicsSystem,
    };
    pub use nalgebra::{vector, Vector2};
    pub use std::{fs::File, io::Write, path::Path};
}

mod del_b;
mod del_c;
mod del_d;
mod del_e;
mod del_f;
mod del_g;

lazy_static! {
    static ref BALL_SNAPSHOT: FreeFallObjectSnapshot = FreeFallObjectSnapshot {
        mass: 0.4,
        charge: 0.0,
        frontal_area: 0.01 * std::f64::consts::PI,
        volume: 0.0,
        position: vector![0.0, 0.0],
        velocity: vector![
            35.0f64.to_radians().cos() * 40.0,
            35.0f64.to_radians().sin() * 40.0
        ],
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

static ref AIRCRAFT_SNAPSHOT: FreeFallObjectSnapshot = FreeFallObjectSnapshot {
    mass: 347450.0,
    frontal_area: 245.5,
    volume: 0.0,
    charge: 0.0,
    position: vector![0.0, 10.0],
    velocity: vector![
        12.5f64.to_radians().cos() * 165.0 * 0.51444,
        12.5f64.to_radians().sin() * 165.0 * 0.51444
    ],
};

static ref AIRCRAFT_RESISTANCE: AirResistanceParameters = AirResistanceParameters {
    c_d: 0.025,
    rho: 1.2,
};
}

fn gravity_force(o: &FreeFallObjectSnapshot, g: Float) -> Vector2<Float> {
    o.mass * g * vector![0.0, -1.0]
}

fn air_drag_force(o: &FreeFallObjectSnapshot, params: AirResistanceParameters) -> Vector2<Float> {
    0.5 * params.c_d * params.rho * o.frontal_area * -1.0 * o.velocity * vector_len(o.velocity)
}

pub fn uppgift_1() {
    del_b::uppgift_b();
    del_c::uppgift_c();
    del_d::uppgift_d();
    del_e::uppgift_e();
    del_f::uppgift_f();
    del_g::uppgift_g();
}

#[derive(Clone, Copy)]
pub struct AirResistanceParameters {
    c_d: Float,
    rho: Float,
}

pub fn run_simulation(
    initial_snapshot: FreeFallObjectSnapshot,
    air_resistance_params: AirResistanceParameters,
    dt: Float,
    output: &mut impl Write,
) {
    let mut object = FreeFallObject {
        snapshot: initial_snapshot,
        forces: vec![
            Box::new(|o| gravity_force(o, 9.82)),
            Box::new(move |o| air_drag_force(o, air_resistance_params)),
        ],
    };

    let mut t = 0.0;

    writeln!(output, "t,x,y").unwrap();

    loop {
        writeln!(
            output,
            "{},{}, {},",
            t, object.snapshot.position[0], object.snapshot.position[1]
        )
        .unwrap();

        object.step_forward(dt);
        t += dt;

        if object.snapshot.position.y < 0.0 {
            break;
        }
    }
}
