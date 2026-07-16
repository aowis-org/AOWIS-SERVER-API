use axum::{
    Router,
    routing::{get, post},
};

use crate::{handlers::hello::greet, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(async || "Hello, world!"))
        .route("/greet", post(greet))
}
