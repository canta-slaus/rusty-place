mod filters;
mod handlers;
mod models;

use models::*;
use warp::Filter;

const WIDTH: usize = 64;
const HEIGHT: usize = 64;
const SIZE: usize = WIDTH * HEIGHT * 3;
const PORT: u16 = 3030;

#[tokio::main]
async fn main() {
    let image = load_from_file("./image");

    let routes = filters::routes(image.clone()).recover(handlers::handle_rejection);

    let (_addr, fut) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], PORT), async move {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to listen to shutdown signal");
        });

    fut.await;
    println!("Shutting down the server");
    write_to_file("./image", &image).await;
}
