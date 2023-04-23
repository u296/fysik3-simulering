use nalgebra::Vector2;

pub type Float = f64;

pub trait PhysicsSystem {
    type SnapShotType;
    fn step_forward(&mut self, dt: Float) -> Self::SnapShotType;
}

pub struct FreeFallObject {
    pub snapshot: FreeFallObjectSnapshot,
    pub forces: Vec<Box<dyn Fn(&FreeFallObjectSnapshot) -> Vector2<Float>>>,
}

#[derive(Debug, Clone, Copy)]
pub struct FreeFallObjectSnapshot {
    pub mass: Float,
    pub charge: Float,
    pub position: Vector2<Float>,
    pub velocity: Vector2<Float>,
}

impl PhysicsSystem for FreeFallObject {
    type SnapShotType = FreeFallObjectSnapshot;
    fn step_forward(&mut self, dt: Float) -> FreeFallObjectSnapshot {
        let force_result: Vector2<Float> = self.forces.iter().map(|f| f(&self.snapshot)).sum();

        let acceleration = force_result / self.snapshot.mass;

        //euler cromers method

        self.snapshot.velocity += acceleration * dt;
        self.snapshot.position += self.snapshot.velocity * dt;
        self.snapshot
    }
}
