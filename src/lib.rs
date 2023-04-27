use std::{future::Future, io::ErrorKind, path::Path, time::Instant};

use nalgebra::SVector;
use tokio::{
    fs,
    task::{self, JoinHandle},
};

pub mod euler;
pub mod euler_cromer;

pub type Float = f64;

pub trait PhysicsSystemSolver {
    type Applied;
    fn step_forward(&mut self) -> Self::Applied;
    fn get_applied(&self) -> Self::Applied;
    fn get_dt(&self) -> Float;
}

pub trait SingleObjectPhysicsSystemSolver<const D: usize>: PhysicsSystemSolver {
    fn get_object<'a>(&'a self) -> &'a FreeFallObject<D>;
}

pub struct FreeFallObject<const D: usize> {
    pub snapshot: FreeFallObjectSnapshot<D>,
    pub forces: Vec<Box<dyn Send + Fn(&FreeFallObjectSnapshot<D>) -> SVector<Float, D>>>,
}

#[derive(Debug, Clone, Copy)]
pub struct FreeFallObjectSnapshot<const D: usize> {
    pub mass: Float,
    pub frontal_area: Float,
    pub volume: Float,
    pub position: SVector<Float, D>,
    pub velocity: SVector<Float, D>,
    pub angular_velocity: SVector<Float, D>,
}

pub struct Step<const D: usize> {
    pub force: SVector<Float, D>,
    pub acceleration: SVector<Float, D>,
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
