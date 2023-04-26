use nalgebra::Vector2;

use crate::{Float, FreeFallObject, HasObject, PhysicsSystem, Step};

pub struct EulerSolver {
    pub object: FreeFallObject,
}

impl EulerSolver {
    pub fn new(object: FreeFallObject) -> Self {
        Self { object }
    }
}

impl PhysicsSystem for EulerSolver {
    type Applied = Step;
    fn step_forward(&mut self, dt: Float) -> Step {
        let applied = self.get_applied();
        let force = applied.force;
        let acceleration = applied.acceleration;

        self.object.snapshot.position += self.object.snapshot.velocity * dt;
        self.object.snapshot.velocity += acceleration * dt;
        Step {
            force,
            acceleration,
        }
    }
    fn get_applied(&self) -> Self::Applied {
        let force: Vector2<Float> = self
            .object
            .forces
            .iter()
            .map(|f| f(&self.object.snapshot))
            .sum();

        let acceleration = force / self.object.snapshot.mass;

        Step {
            force,
            acceleration,
        }
    }
}

impl HasObject for EulerSolver {
    fn get_object<'a>(&'a self) -> &'a FreeFallObject {
        &self.object
    }
}
