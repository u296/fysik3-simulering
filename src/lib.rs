use std::{future::Future, io::ErrorKind, path::Path, time::Instant};

use nalgebra::SVector;
use tokio::{
    fs,
    task::{self, JoinHandle},
};

pub mod data;
pub mod simulation;
pub mod solver;

pub type Float = f64;

pub type ForceFunction<const D: usize> =
    Box<dyn 'static + Send + Fn(&FreeFallObjectSnapshot<D>) -> SVector<Float, D>>;

#[derive(Debug, Clone, Copy)]
pub struct FreeFallObjectSnapshot<const D: usize> {
    pub mass: Float,
    pub frontal_area: Float,
    pub volume: Float,
    pub position: SVector<Float, D>,
    pub velocity: SVector<Float, D>,
    pub angular_velocity: SVector<Float, D>,
}

pub struct FreeFallObject<const D: usize> {
    pub snapshot: FreeFallObjectSnapshot<D>,
    pub forces: Vec<ForceFunction<D>>,
}

pub struct AppliedDynamics<const D: usize> {
    pub force: SVector<Float, D>,
    pub acceleration: SVector<Float, D>,
}

pub async fn ensure_dir_exists(p: impl AsRef<Path>) {
    if let Err(e) = fs::create_dir_all(p).await {
        match e.kind() {
            ErrorKind::AlreadyExists => (),
            _ => panic!("failed to create dir: {e}"),
        }
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
    task::spawn(async move {
        let start = Instant::now();
        let result = f().await;

        println!(
            "task {name} finished in {:.3} s",
            Instant::now().duration_since(start).as_secs_f32()
        );
        result
    })
}
