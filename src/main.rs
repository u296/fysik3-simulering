use std::fs::File;
use std::io::Write;

use fysik3_simulering::{Float, FreeFallObject, FreeFallObjectSnapshot, PhysicsSystem};
use nalgebra::{vector, Vector2};
use uppgift_1::uppgift_1;

mod uppgift_1;

fn vector_len(v: Vector2<Float>) -> Float {
    (v[0].powi(2) + v[1].powi(2)).sqrt()
}

fn main() {
    uppgift_1()
}
