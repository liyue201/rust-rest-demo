
use axum::{
    Router,
    routing::{get, post},
};

use crate::server::user::login;

pub fn setup_router() -> Router {
    Router::new().route("/login", post(login))
}