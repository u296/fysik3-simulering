use fysik3_simulering::Float;
use nalgebra::Vector2;
use uppgift_1::uppgift_1;
use uppgift_2::uppgift_2;

mod uppgift_1;
mod uppgift_2;
mod uppgift_3;

fn vector_len(v: Vector2<Float>) -> Float {
    (v[0].powi(2) + v[1].powi(2)).sqrt()
}

#[tokio::main]
async fn main() {
    uppgift_1().await;
    //uppgift_2().await;
}
