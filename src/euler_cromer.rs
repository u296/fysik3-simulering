use nalgebra::Vector2;

use crate::{Float, FreeFallObject, PhysicsSystemSolver, SingleObjectPhysicsSystemSolver, Step};

pub struct EulerCromerSolver {
    pub object: FreeFallObject,
}

impl EulerCromerSolver {
    pub fn new(object: FreeFallObject) -> Self {
        Self { object }
    }
}

impl PhysicsSystemSolver for EulerCromerSolver {
    type Applied = Step;
    fn step_forward(&mut self, dt: Float) -> Step {
        let applied = self.get_applied();
        let force = applied.force;
        let acceleration = applied.acceleration;

        //euler cromers method

        self.object.snapshot.velocity += acceleration * dt;
        self.object.snapshot.position += self.object.snapshot.velocity * dt;
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

impl SingleObjectPhysicsSystemSolver for EulerCromerSolver {
    fn get_object<'a>(&'a self) -> &'a FreeFallObject {
        &self.object
    }
}
