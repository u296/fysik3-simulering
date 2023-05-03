use nalgebra::SVector;

use crate::{AppliedDynamics, Body, Float, StepChanges};

use super::{PhysicsSystemSolver, SingleObjectPhysicsSystemSolver, Step};

pub struct EulerSolver<const D: usize> {
    pub object: Body<D>,
    dt: Float,
}

impl<const D: usize> EulerSolver<D> {
    pub fn new(object: Body<D>, dt: Float) -> Self {
        Self { object, dt }
    }
}

impl<const D: usize> PhysicsSystemSolver for EulerSolver<D> {
    type StepType = Step<D>;
    fn step_forward(&self) -> Self::StepType {
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

        let applied = AppliedDynamics {
            force,
            acceleration,
            torque,
            angular_acceleration,
        };

        let mut new_state = self.get_object().snapshot;

        let delta_s = new_state.velocity * self.dt;
        new_state.position += delta_s;

        let delta_v = acceleration * self.dt;
        new_state.velocity += delta_v;

        new_state.angular_velocity += angular_acceleration * self.dt;

        Self::StepType {
            time: self.dt,
            applied,
            deltas: StepChanges { delta_s, delta_v },
            new_state,
        }
    }
}

impl<const D: usize> SingleObjectPhysicsSystemSolver<D> for EulerSolver<D> {
    fn get_object(&self) -> &Body<D> {
        &self.object
    }

    fn get_object_mut(&mut self) -> &mut Body<D> {
        &mut self.object
    }
}
