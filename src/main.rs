mod handlers;
mod models;
mod routes;
mod state;
use tokio::net::TcpListener;

use crate::{routes::app, state::AppState};

#[tokio::main]
async fn main() {
    let state = AppState::from_json("seed/products.json");
    let app = app(state);

    let listener = TcpListener::bind("0.0.0.0:8869").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
