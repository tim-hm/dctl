use super::theme::ThemeManager;
use axum::body::Empty;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use log::info;
use serde::{Deserialize, Serialize};

pub fn build_routes() -> Router {
    Router::new()
        .route("/theme/toggle", get(theme_toggle))
        .route("/theme/set", post(theme_set))
}

async fn theme_toggle() -> impl IntoResponse {
    let manager = ThemeManager::new();
    let result = manager.toggle();

    match result {
        Ok(theme) => info!("Theme changed to {:?}", theme),
        Err(error) => log::error!("{}", error),
    }

    Response::new(Empty::new())
}

async fn theme_set(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
