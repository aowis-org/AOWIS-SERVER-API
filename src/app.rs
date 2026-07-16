use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use tokio::fs;
use tower_http::trace::TraceLayer;

use crate::{routes::hello_world, state::AppState};

pub fn app_router() -> Router {
    let state = AppState {};

    let router = Router::new()
        .nest("/hello", hello_world::router())
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    // Show this page only in [DEBUG] builds
    if cfg!(debug_assertions) {
        router.route("/", get(root))
    } else {
        router
    }
}

/// Only used in [DEBUG] builds
async fn root() -> impl IntoResponse {
    let index_page = fs::read_to_string("pages/index.html").await.unwrap();

    Html(index_page)
}
