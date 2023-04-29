use tokio::join;

use fysik3_simulering::spawn_timed_task;
use uppgift_1::uppgift_1;
use uppgift_2::uppgift_2;
use uppgift_3::uppgift_3;
use uppgift_extra_1::uppgift_extra_1;
use uppgift_extra_2::uppgift_extra_2;

mod uppgift_1;
mod uppgift_2;
mod uppgift_3;
mod uppgift_extra_1;
mod uppgift_extra_2;

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main());
}

async fn async_main() {
    spawn_timed_task("main", || async {
        let (a, b, c, d, e) = join!(
            spawn_timed_task("uppgift 1", uppgift_1),
            spawn_timed_task("uppgift 2", uppgift_2),
            spawn_timed_task("uppgift 3", uppgift_3),
            spawn_timed_task("uppgift extra 1", uppgift_extra_1),
            spawn_timed_task("uppgift extra 2", uppgift_extra_2),
        );
        [a, b, c, d, e].into_iter().for_each(|x| x.unwrap());
    })
    .await
    .unwrap();
}
