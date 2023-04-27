use std::{future::Future, io::ErrorKind, path::Path, time::Instant};

use nalgebra::Vector2;
use tokio::{
    fs,
    task::{self, JoinHandle},
};

pub mod euler;
pub mod euler_cromer;

pub type Float = f64;

pub trait PhysicsSystemSolver {
    type Applied;
    fn step_forward(&mut self, dt: Float) -> Self::Applied;
    fn get_applied(&self) -> Self::Applied;
}

pub trait SingleObjectPhysicsSystemSolver: PhysicsSystemSolver {
    fn get_object<'a>(&'a self) -> &'a FreeFallObject;
}

pub struct FreeFallObject {
    pub snapshot: FreeFallObjectSnapshot,
    pub forces: Vec<Box<dyn Send + Fn(&FreeFallObjectSnapshot) -> Vector2<Float>>>,
}

#[derive(Debug, Clone, Copy)]
pub struct FreeFallObjectSnapshot {
    pub mass: Float,
    pub charge: Float,
    pub frontal_area: Float,
    pub volume: Float,
    pub position: Vector2<Float>,
    pub velocity: Vector2<Float>,
}

pub struct Step {
    pub force: Vector2<Float>,
    pub acceleration: Vector2<Float>,
}

pub async fn ensure_dir_exists(p: impl AsRef<Path>) {
    match fs::create_dir_all(p).await {
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => (),
            _ => panic!("failed to create dir: kind = {} error: {e}", e.kind()),
        },
        _ => (),
    }
}

pub fn spawn_timed_task<
    T: Send + Sync + 'static,
    F: Future<Output = T> + Send + 'static,
    FN: Send + FnOnce() -> F + 'static,
>(
    name: &'static str,
    f: FN,
) -> JoinHandle<T> {
    task::spawn((move || async move {
        let start = Instant::now();
        let result = f().await;

        println!(
            "task {name} finished in {:.3} s",
            Instant::now().duration_since(start).as_secs_f32()
        );
        result
    })())
}
