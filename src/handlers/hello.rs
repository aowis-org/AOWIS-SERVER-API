use axum::{Form, response::IntoResponse};

use crate::dto::hello::Greet;

pub async fn greet(Form(greet): Form<Greet>) -> impl IntoResponse {
    format!("Hello, {}!", greet.name)
}
