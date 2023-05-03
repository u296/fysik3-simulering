use tokio::io::AsyncWrite;

use crate::{
    data::DataLogger,
    solver::{SingleObjectPhysicsSystemSolver, Step},
};

pub async fn run_simulation<
    DataType: DataLogger<D, N, Step<D>, UserType, W> + Send,
    const D: usize,
    const N: usize,
    UserType,
    Solver: SingleObjectPhysicsSystemSolver<D, StepType = Step<D>>,
    W: AsyncWrite + Unpin + Send + Sync,
>(
    mut solver: Solver,
    user: UserType,
    mut datalogger: DataType,
) {
    let mut t = 0.0;
    let mut datapoints = Vec::new();

    loop {
        let step = solver.step_forward();
        {
            let snapshot = &solver.get_object().snapshot;
            datapoints.push(datalogger.new_datapoint(t, snapshot, &step, &user));

            if datalogger.should_end(t, &snapshot, &step, &datapoints, &user) {
                break;
            }
        }

        solver.get_object_mut().snapshot = step.new_state;

        t += solver.step_forward().time;
    }

    DataType::write_data(&datapoints, datalogger.get_output()).await;
}
