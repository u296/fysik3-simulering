use nalgebra::SVector;

use crate::{AppliedDynamics, Float, FreeFallObject};

use super::{PhysicsSystemSolver, SingleObjectPhysicsSystemSolver, Step};

pub struct EulerSolver<const D: usize> {
    pub object: FreeFallObject<D>,
    dt: Float,
}

impl<const D: usize> EulerSolver<D> {
    pub fn new(object: FreeFallObject<D>, dt: Float) -> Self {
        Self { object, dt }
    }
}

impl<const D: usize> PhysicsSystemSolver for EulerSolver<D> {
    type Applied = AppliedDynamics<D>;
    fn step_forward(&mut self) -> Step<AppliedDynamics<D>> {
        let applied = self.get_applied();

        self.object.snapshot.position += self.object.snapshot.velocity * self.dt;
        self.object.snapshot.velocity += applied.acceleration * self.dt;

        self.object.snapshot.angular_velocity += applied.angular_acceleration * self.dt;

        Step {
            time: self.dt,
            applied,
        }
    }
    fn get_applied(&self) -> Self::Applied {
        let force: SVector<Float, D> = self
            .object
            .forces
            .iter()
            .map(|f| f(&self.object.snapshot))
            .sum();
        let torque: SVector<Float, D> = self
            .object
            .torques
            .iter()
            .map(|f| f(&self.object.snapshot))
            .sum();

        let acceleration = force / self.object.snapshot.mass;
        let angular_acceleration = torque / self.object.snapshot.moment_of_inertia;

        AppliedDynamics {
            force,
            acceleration,
            torque,
            angular_acceleration,
        }
    }

    fn get_dt(&self) -> Float {
        self.dt
    }
}

impl<const D: usize> SingleObjectPhysicsSystemSolver<D> for EulerSolver<D> {
    fn get_object(&self) -> &FreeFallObject<D> {
        &self.object
    }
}
