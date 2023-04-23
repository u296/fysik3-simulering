use std::{fs, io::ErrorKind, path::Path};

use nalgebra::Vector2;

pub type Float = f64;

pub trait PhysicsSystem {
    type StepResultType;
    fn step_forward(&mut self, dt: Float) -> Self::StepResultType;
}

pub struct FreeFallObject {
    pub snapshot: FreeFallObjectSnapshot,
    pub forces: Vec<Box<dyn Fn(&FreeFallObjectSnapshot) -> Vector2<Float>>>,
}

#[derive(Debug, Clone, Copy)]
pub struct FreeFallObjectSnapshot {
    pub mass: Float,
    pub charge: Float,
    pub frontal_area: Float,
    pub volume: Float,
    pub position: Vector2<Float>,
    pub velocity: Vector2<Float>,
}

impl PhysicsSystem for FreeFallObject {
    type StepResultType = Step;
    fn step_forward(&mut self, dt: Float) -> Step {
        let force_result: Vector2<Float> = self.forces.iter().map(|f| f(&self.snapshot)).sum();

        let acceleration = force_result / self.snapshot.mass;

        //euler cromers method

        self.snapshot.velocity += acceleration * dt;
        self.snapshot.position += self.snapshot.velocity * dt;
        Step {
            force: force_result,
            acceleration,
        }
    }
}

pub struct Step {
    pub force: Vector2<Float>,
    pub acceleration: Vector2<Float>,
}

pub fn ensure_dir_exists(p: impl AsRef<Path>) {
    match fs::create_dir_all(p) {
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => (),
            _ => panic!("failed to create dir: {e}"),
        },
        _ => (),
    }
}
