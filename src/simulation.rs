use tokio::io::AsyncWrite;

use crate::{
    data::Data, solver::SingleObjectPhysicsSystemSolver, Float, ForceFunction, FreeFallObject,
    FreeFallObjectSnapshot,
};

pub async fn run_simulation<
    DataType: Data<D, N, Solver::Applied, UserType> + Send,
    const D: usize,
    const N: usize,
    UserType,
    Solver: SingleObjectPhysicsSystemSolver<D>,
    SolverNewFn: Fn(FreeFallObject<D>, Float) -> Solver,
    Output: AsyncWrite + Unpin + Send,
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

    DataType::write_data(&datapoints, output).await;
}
