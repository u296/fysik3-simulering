use nalgebra::Vector2;

use crate::{Float, FreeFallObject, PhysicsSystemSolver, SingleObjectPhysicsSystemSolver, Step};

pub struct EulerSolver {
    pub object: FreeFallObject,
    dt: Float,
}

impl EulerSolver {
    pub fn new(object: FreeFallObject, dt: Float) -> Self {
        Self { object, dt }
    }
}

impl PhysicsSystemSolver for EulerSolver {
    type Applied = Step;
    fn step_forward(&mut self) -> Step {
        let applied = self.get_applied();
        let force = applied.force;
        let acceleration = applied.acceleration;

        self.object.snapshot.position += self.object.snapshot.velocity * self.dt;
        self.object.snapshot.velocity += acceleration * self.dt;
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

    fn get_dt(&self) -> Float {
        self.dt
    }
}

impl SingleObjectPhysicsSystemSolver for EulerSolver {
    fn get_object<'a>(&'a self) -> &'a FreeFallObject {
        &self.object
    }
}
