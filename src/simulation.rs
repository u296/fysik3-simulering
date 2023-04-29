use tokio::io::AsyncWrite;

use crate::{data::Data, solver::SingleObjectPhysicsSystemSolver};

pub async fn run_simulation<
    DataType: Data<D, N, Solver::Applied, UserType> + Send,
    const D: usize,
    const N: usize,
    UserType,
    Solver: SingleObjectPhysicsSystemSolver<D>,
    Output: AsyncWrite + Unpin + Send,
>(
    mut solver: Solver,
    user: UserType,
    output: &mut Output,
) {
    let mut t = 0.0;
    let mut datapoints = Vec::new();

    loop {
        let snapshot = &solver.get_object().snapshot;

        let applied = solver.get_applied();

        datapoints.push(DataType::new_datapoint(t, snapshot, &applied, &user));

        if DataType::should_end(t, snapshot, &applied, &datapoints, &user) {
            break;
        }

        t += solver.step_forward().time;
    }

    DataType::write_data(&datapoints, output).await;
}
