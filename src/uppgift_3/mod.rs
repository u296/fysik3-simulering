mod prelude {
    pub use super::{run_simulation, DEFAULT_INIT_SNAPSHOT, DEFAULT_K};
    pub use fysik3_simulering::{
        ensure_dir_exists, euler::EulerSolver, euler_cromer::EulerCromerSolver, Float,
        FreeFallObject, FreeFallObjectSnapshot, PhysicsSystem,
    };
    pub use nalgebra::{vector, Vector2};
    pub use tokio::fs::File;
}

use fysik3_simulering::{spawn_timed_task, HasObject, Step};
use prelude::*;
use tokio::{
    io::{AsyncWrite, AsyncWriteExt, BufWriter},
    join,
};

use del_a::uppgift_a;
use del_c::uppgift_c;
use del_d::uppgift_d;
use del_e::uppgift_e;
use uppgift_extra_b::uppgift_extra_b;
use uppgift_extra_c::uppgift_extra_c;

use crate::vector_len;

mod del_a;
mod del_c;
mod del_d;
mod del_e;
mod uppgift_extra_b;
mod uppgift_extra_c;

const NUM_DATAPOINTS: usize = 2000;
pub const DEFAULT_INIT_SNAPSHOT: FreeFallObjectSnapshot = FreeFallObjectSnapshot {
    mass: 1.0,
    charge: 0.0,
    frontal_area: 0.0,
    volume: 0.0,
    position: vector![10.0, 0.0],
    velocity: vector![0.0, 0.0],
};
pub const DEFAULT_K: Float = 100.0;

pub async fn uppgift_3() {
    let (a, c, d, e, extra_b, extra_c) = join!(
        spawn_timed_task("3-a", uppgift_a),
        spawn_timed_task("3-c", uppgift_c),
        spawn_timed_task("3-d", uppgift_d),
        spawn_timed_task("3-e", uppgift_e),
        spawn_timed_task("3-extra-b", uppgift_extra_b),
        spawn_timed_task("3-extra-c", uppgift_extra_c),
    );
    [a, c, d, e, extra_b, extra_c]
        .into_iter()
        .for_each(|x| x.unwrap());
}

pub async fn run_simulation<W: Unpin + AsyncWrite, P: PhysicsSystem<Applied = Step> + HasObject>(
    init_snapshot: FreeFallObjectSnapshot,
    k: Float,
    r: Float,
    dt: Float,
    output: &mut W,
    solver_new: impl Fn(FreeFallObject) -> P,
) {
    let mut solver = solver_new(FreeFallObject {
        snapshot: init_snapshot,
        forces: vec![
            Box::new(move |o| o.position * -k),
            Box::new(move |o| o.velocity * -r),
        ],
    });

    let mut t = 0.0;
    let mut datapoints = Vec::new();

    loop {
        let potential_energy = k * vector_len(solver.get_object().snapshot.position).powi(2) / 2.0;
        let kinetic_energy = solver.get_object().snapshot.mass
            * vector_len(solver.get_object().snapshot.velocity).powi(2)
            / 2.0;
        let mech_energy = potential_energy + kinetic_energy;

        let a = solver.get_applied().acceleration[0];

        datapoints.push([
            t,
            solver.get_object().snapshot.position[0],
            solver.get_object().snapshot.velocity[0],
            a,
            mech_energy,
        ]);

        solver.step_forward(dt).acceleration;
        t += dt;

        if t > 10.0 {
            break;
        }
    }

    let mut output_writer = BufWriter::new(output);

    output_writer.write_all(b"t,x,v,a,E_mek\n").await.unwrap();

    if datapoints.len() > NUM_DATAPOINTS {
        let mut index = 0.0;

        let step_size = datapoints.len() as f32 / NUM_DATAPOINTS as f32;

        while let Some(datapoint) = datapoints.get(index as usize) {
            let buf = format!(
                "{}, {}, {}, {}, {}\n",
                datapoint[0], datapoint[1], datapoint[2], datapoint[3], datapoint[4]
            );
            output_writer.write_all(buf.as_bytes()).await.unwrap();
            index += step_size;
        }
    } else {
        for datapoint in datapoints {
            let buf = format!(
                "{}, {}, {}, {}, {}\n",
                datapoint[0], datapoint[1], datapoint[2], datapoint[3], datapoint[4]
            );
            output_writer.write_all(buf.as_bytes()).await.unwrap();
        }
    }

    output_writer.flush().await.unwrap();
}
