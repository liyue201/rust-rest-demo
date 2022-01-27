use axum::{
    Router,
    routing::{get, post},
};

use crate::server::user::{login, register};

pub fn setup_router() -> Router {
    Router::new()
        .route("/v1/register", post(register))
        .route("/v1/login", post(login))
}