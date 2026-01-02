mod database;
mod generator;
mod handlers;
mod models;

use axum::{
    Router,
    routing::{get, post, put},
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use database::db_pool;
use generator::AnimalNameGenerator;
use handlers::{AppState, add, animals, home, update};

#[tokio::main]
async fn main() {
    let state = AppState {
        db: db_pool().await,
        generator: AnimalNameGenerator::new(),
    };

    let router = Router::new()
        .route("/", get(home))
        .route("/animals", post(add))
        .route("/animals", put(update))
        .route("/animals", get(animals))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_headers(Any)
                .allow_origin(Any),
        )
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let tcp = TcpListener::bind(&addr).await.unwrap();

    println!("start server on http://0.0.0.0:8000");

    axum::serve(tcp, router).await.unwrap();
}
