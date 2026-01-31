use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main () {
    let r1 = create_router_1();
    let r2 = create_router_2();

    let r = Router::merge(r1, r2);

    let listener = TcpListener::bind("0.0.0.0:7021").await.unwrap();
    axum::serve(listener, r).await.unwrap();
}

fn create_router_1 () -> Router<()> {
    Router::new().route("/hello", get(|| async { "Hello World" }))
}

fn create_router_2 () -> Router<()> {
    Router::new().route("/goodbye", get(|| async { "Goodbye, cruel world" }))
}
