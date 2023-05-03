use crate::{AppliedDynamics, Body, BodySnapshot, Float, StepChanges};

mod euler;
mod euler_cromer;

pub use euler::EulerSolver;
pub use euler_cromer::EulerCromerSolver;

pub struct Step<const D: usize> {
    pub time: Float,
    pub applied: AppliedDynamics<D>,
    pub deltas: StepChanges<D>,
    pub new_state: BodySnapshot<D>,
}

pub trait PhysicsSystemSolver {
    type StepType;
    fn step_forward(&self) -> Self::StepType;
}

pub trait SingleObjectPhysicsSystemSolver<const D: usize>: PhysicsSystemSolver {
    fn get_object(&self) -> &Body<D>;
    fn get_object_mut(&mut self) -> &mut Body<D>;
}
