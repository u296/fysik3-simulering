use std::{future::Future, io::ErrorKind, path::Path, time::Instant};

use nalgebra::SVector;
use tokio::{
    fs,
    io::{AsyncWrite, AsyncWriteExt, BufWriter},
    task::{self, JoinHandle},
};

pub mod euler;
pub mod euler_cromer;

const MAX_DATAPOINTS: usize = 2000;

pub type Float = f64;

pub type ForceFunction<const D: usize> =
    Box<dyn 'static + Send + Fn(&FreeFallObjectSnapshot<D>) -> SVector<Float, D>>;

pub trait PhysicsSystemSolver {
    type Applied;
    fn step_forward(&mut self) -> Self::Applied;
    fn get_applied(&self) -> Self::Applied;
    fn get_dt(&self) -> Float;
}

pub trait SingleObjectPhysicsSystemSolver<const D: usize>: PhysicsSystemSolver {
    fn get_object(&self) -> &FreeFallObject<D>;
}

pub struct FreeFallObject<const D: usize> {
    pub snapshot: FreeFallObjectSnapshot<D>,
    pub forces: Vec<ForceFunction<D>>,
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

async fn write_datapoint<W: AsyncWrite + Unpin, const N: usize>(
    output: &mut W,
    datapoint: [Float; N],
) {
    let mut buf = datapoint.map(|x| ToString::to_string(&x)).join(", ");
    buf.push('\n');

    output.write_all(buf.as_bytes()).await.unwrap();
}

pub trait Data<const D: usize, const N: usize, AppliedType, UserType> {
    fn new_datapoint(
        time: Float,
        object: &FreeFallObjectSnapshot<D>,
        applied: &AppliedType,
        user: &UserType,
    ) -> [Float; N];
    fn column_names() -> [&'static str; N];
    fn should_end(
        time: Float,
        object: &FreeFallObjectSnapshot<D>,
        applied: &AppliedType,
        current_data: &[[Float; N]],
        user: &UserType,
    ) -> bool;
}

pub async fn run_simulation<
    DataType: Data<D, N, Solver::Applied, UserType>,
    const D: usize,
    const N: usize,
    UserType,
    Solver: SingleObjectPhysicsSystemSolver<D>,
    SolverNewFn: Fn(FreeFallObject<D>, Float) -> Solver,
    Output: AsyncWrite + Unpin,
>(
    solver_new: SolverNewFn,
    initial_snapshot: FreeFallObjectSnapshot<D>,
    forces: Vec<ForceFunction<D>>,
    dt: Float,
    user: UserType,
    output: &mut Output,
) {
    let mut solver = solver_new(
        FreeFallObject {
            snapshot: initial_snapshot,
            forces,
        },
        dt,
    );

    let mut t = 0.0;
    let mut datapoints = Vec::new();

    loop {
        let snapshot = &solver.get_object().snapshot;

        let applied = solver.get_applied();

        datapoints.push(DataType::new_datapoint(t, snapshot, &applied, &user));

        if DataType::should_end(t, snapshot, &applied, &datapoints, &user) {
            break;
        }

        solver.step_forward();
        t += dt;
    }

    let mut output_writer = BufWriter::new(output);

    let first_row = {
        let mut b = DataType::column_names().join(", ");
        b.push('\n');
        b
    };

    output_writer.write_all(first_row.as_bytes()).await.unwrap();

    if datapoints.len() > MAX_DATAPOINTS {
        let mut index = 0.0;

        let step_size = datapoints.len() as f32 / MAX_DATAPOINTS as f32;

        while let Some(datapoint) = datapoints.get(index as usize) {
            write_datapoint(&mut output_writer, *datapoint).await;
            index += step_size;
        }
    } else {
        for datapoint in datapoints {
            write_datapoint(&mut output_writer, datapoint).await;
        }
    }

    output_writer.flush().await.unwrap()
}
