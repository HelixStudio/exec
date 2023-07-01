mod controller;
mod models;
mod utils;

use axum::{routing::get, Router};
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    println!(
        "Current working directory is {}",
        env::current_dir().unwrap().display()
    );

    let app = Router::new()
        .route("/api/v1/status", get(controller::status))
        .route("/api/v1/runtimes", get(controller::placeholder))
        .route("/api/v1/execute", get(controller::placeholder))
        .route("/", get(|| async { "Service is up!" }));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
