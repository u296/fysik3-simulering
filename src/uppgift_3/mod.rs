mod prelude {
    pub use super::run_simulation;
    pub use fysik3_simulering::{
        ensure_dir_exists, Float, FreeFallObject, FreeFallObjectSnapshot, PhysicsSystem,
    };
    pub use nalgebra::{vector, Vector2};
    pub use tokio::fs::File;
}

use fysik3_simulering::spawn_timed_task;
use prelude::*;
use tokio::{
    io::{AsyncWrite, AsyncWriteExt, BufWriter},
    join,
};

use del_a::uppgift_a;
use del_c::uppgift_c;
use del_d::uppgift_d;

mod del_a;
mod del_c;
mod del_d;

const NUM_DATAPOINTS: usize = 2000;

pub async fn uppgift_3() {
    let (a, c, d) = join!(
        spawn_timed_task("3-a", uppgift_a),
        spawn_timed_task("3-c", uppgift_c),
        spawn_timed_task("3-d", uppgift_d),
    );
    [a, c, d].into_iter().for_each(|x| x.unwrap());
}

pub async fn run_simulation<W: Unpin + AsyncWrite>(
    init_snapshot: FreeFallObjectSnapshot,
    k: Float,
    r: Float,
    dt: Float,
    output: &mut W,
) {
    let mut object = FreeFallObject {
        snapshot: init_snapshot,
        forces: vec![
            Box::new(move |o| o.position * -k),
            Box::new(move |o| o.velocity * -r),
        ],
    };

    let mut t = 0.0;
    let mut datapoints = Vec::new();

    loop {
        let a = object.step_forward(dt).acceleration;
        t += dt;

        datapoints.push([
            t,
            object.snapshot.position[0],
            object.snapshot.velocity[0],
            a[0],
        ]);

        if t > 10.0 {
            break;
        }
    }

    let mut output_writer = BufWriter::new(output);

    output_writer.write_all(b"t,x,v,a\n").await.unwrap();

    if datapoints.len() > NUM_DATAPOINTS {
        let mut index = 0.0;

        let step_size = datapoints.len() as f32 / NUM_DATAPOINTS as f32;

        while let Some(datapoint) = datapoints.get(index as usize) {
            let buf = format!(
                "{}, {}, {}, {}\n",
                datapoint[0], datapoint[1], datapoint[2], datapoint[3]
            );
            output_writer.write_all(buf.as_bytes()).await.unwrap();
            index += step_size;
        }
    } else {
        for datapoint in datapoints {
            let buf = format!(
                "{}, {}, {}, {}\n",
                datapoint[0], datapoint[1], datapoint[2], datapoint[3]
            );
            output_writer.write_all(buf.as_bytes()).await.unwrap();
        }
    }
}
