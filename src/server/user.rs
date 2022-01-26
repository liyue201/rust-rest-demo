
use axum::{
    Json,
    response::IntoResponse,
    Router,
    routing::get,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    id: i32,
    username: String,
}

#[derive(Serialize)]
pub struct Response {
    code: i32,
    data: User,
}

pub async fn login() -> Json<Response> {
    let user = User {
        id: 10,
        username: String::from("aa"),
    };
    let resp = Response {
        code: 0,
        data: user,
    };
    Json(resp)
}
