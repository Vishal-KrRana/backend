use crate::models::Product;

use axum::{Json, extract::State, http::StatusCode};

use crate::state::AppState;

pub async fn create(State(state): State<AppState>, Json(data): Json<Product>) -> StatusCode {
    {
        state.products.lock().unwrap().push(data.clone());
    }
    StatusCode::CREATED
}

pub async fn list(State(state): State<AppState>) -> Json<Vec<Product>> {
    let products: Vec<Product> = state.products.lock().unwrap().clone();
    Json(products)
}
