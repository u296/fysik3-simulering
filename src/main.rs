use fysik3_simulering::{spawn_timed_task, Float};
use nalgebra::Vector2;
use tokio::join;
use uppgift_1::uppgift_1;
use uppgift_2::uppgift_2;
use uppgift_3::uppgift_3;

mod uppgift_1;
mod uppgift_2;
mod uppgift_3;

fn vector_len(v: Vector2<Float>) -> Float {
    (v[0].powi(2) + v[1].powi(2)).sqrt()
}

#[tokio::main]
async fn main() {
    let (a, b, c) = join!(
        spawn_timed_task("uppgift 1", uppgift_1),
        spawn_timed_task("uppgift 2", uppgift_2),
        spawn_timed_task("uppgift 3", uppgift_3)
    );
    [a, b, c].into_iter().for_each(|x| x.unwrap());
}
