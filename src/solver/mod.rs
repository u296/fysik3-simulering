use crate::{Body, Float};

mod euler;
mod euler_cromer;

pub use euler::EulerSolver;
pub use euler_cromer::EulerCromerSolver;

pub struct Step<T> {
    pub time: Float,
    pub applied: T,
}

pub trait PhysicsSystemSolver {
    type Applied;
    fn step_forward(&mut self) -> Step<Self::Applied>;
    fn get_applied(&self) -> Self::Applied;
}

pub trait SingleObjectPhysicsSystemSolver<const D: usize>: PhysicsSystemSolver {
    fn get_object(&self) -> &Body<D>;
}
