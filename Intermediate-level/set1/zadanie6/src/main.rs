
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(|| async { "WItaj" }))
    .route("/hello", get(|| async { "raz dwa trzy" }))
    .route("/status", get(|| async { "Serwer dziala" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server is running on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

