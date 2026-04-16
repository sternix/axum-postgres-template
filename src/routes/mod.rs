use axum::{Router, routing::get};

pub mod health_check;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/health_check", get(health_check::health_check))
}
