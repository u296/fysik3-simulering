use nalgebra::SVector;

use crate::{
    AppliedDynamics, Float, FreeFallObject, PhysicsSystemSolver, SingleObjectPhysicsSystemSolver,
};

pub struct EulerCromerSolver<const D: usize> {
    pub object: FreeFallObject<D>,
    dt: Float,
}

impl<const D: usize> EulerCromerSolver<D> {
    pub fn new(object: FreeFallObject<D>, dt: Float) -> Self {
        Self { object, dt }
    }
}

impl<const D: usize> PhysicsSystemSolver for EulerCromerSolver<D> {
    type Applied = AppliedDynamics<D>;
    fn step_forward(&mut self) -> AppliedDynamics<D> {
        let applied = self.get_applied();
        let force = applied.force;
        let acceleration = applied.acceleration;

        //euler cromers method

        self.object.snapshot.velocity += acceleration * self.dt;
        self.object.snapshot.position += self.object.snapshot.velocity * self.dt;
        AppliedDynamics {
            force,
            acceleration,
        }
    }
    fn get_applied(&self) -> Self::Applied {
        let force: SVector<Float, D> = self
            .object
            .forces
            .iter()
            .map(|f| f(&self.object.snapshot))
            .sum();

        let acceleration = force / self.object.snapshot.mass;

        AppliedDynamics {
            force,
            acceleration,
        }
    }

    fn get_dt(&self) -> Float {
        self.dt
    }
}

impl<const D: usize> SingleObjectPhysicsSystemSolver<D> for EulerCromerSolver<D> {
    fn get_object(&self) -> &FreeFallObject<D> {
        &self.object
    }
}
