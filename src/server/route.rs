
use axum::{
    Router,
    routing::get,
};

use crate::server::user::login;

pub fn setup_router() -> Router {
    Router::new().route("/", get(login))
}