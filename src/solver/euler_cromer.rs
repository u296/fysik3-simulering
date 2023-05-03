use nalgebra::SVector;

use crate::{AppliedDynamics, Body, Float, StepChanges};

use super::{PhysicsSystemSolver, SingleObjectPhysicsSystemSolver, Step};

pub struct EulerCromerSolver<const D: usize> {
    pub object: Body<D>,
    dt: Float,
}

impl<const D: usize> EulerCromerSolver<D> {
    pub fn new(object: Body<D>, dt: Float) -> Self {
        Self { object, dt }
    }
}

impl<const D: usize> PhysicsSystemSolver for EulerCromerSolver<D> {
    type StepType = Step<D>;
    fn step_forward(&self) -> Step<D> {
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

        //euler cromers method

        let mut new_state = self.object.snapshot;

        let delta_v = acceleration * self.dt;
        new_state.velocity += delta_v;

        let delta_s = new_state.velocity * self.dt;
        new_state.position += delta_s;

        new_state.angular_velocity += angular_acceleration * self.dt;

        Step {
            time: self.dt,
            applied,
            deltas: StepChanges { delta_s, delta_v },
            new_state,
        }
    }
}

impl<const D: usize> SingleObjectPhysicsSystemSolver<D> for EulerCromerSolver<D> {
    fn get_object(&self) -> &Body<D> {
        &self.object
    }

    fn get_object_mut(&mut self) -> &mut Body<D> {
        &mut self.object
    }
}
