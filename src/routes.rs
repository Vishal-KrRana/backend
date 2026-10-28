use crate::{handlers::products, state::AppState};
use axum::{Router, routing};
pub fn app(state: AppState) -> Router {
    let app = Router::new()
        .route(
            "/api/products",
            routing::get(products::list).post(products::create),
        )
        .with_state(state);
    app
}
