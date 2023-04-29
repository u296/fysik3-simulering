use crate::{Float, FreeFallObject};

mod euler;
mod euler_cromer;

pub use euler::EulerSolver;
pub use euler_cromer::EulerCromerSolver;

pub trait PhysicsSystemSolver {
    type Applied;
    fn step_forward(&mut self) -> Self::Applied;
    fn get_applied(&self) -> Self::Applied;
    fn get_dt(&self) -> Float;
}

pub trait SingleObjectPhysicsSystemSolver<const D: usize>: PhysicsSystemSolver {
    fn get_object(&self) -> &FreeFallObject<D>;
}
