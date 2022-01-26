use axum::{
    http::*,
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct User {
    id: i32,
    username: String,
}

#[derive(Serialize)]
pub struct TResponse<T> {
    code: i32,
    data: T,
}

impl<T> IntoResponse for TResponse<T>
    where
        T: Serialize,
{
    fn into_response(self) -> Response {
        let body = Json(json!(self));
        (StatusCode::from_u16(200).unwrap(), body).into_response()
    }
}

pub async fn login() -> impl IntoResponse {
    let user = User {
        id: 10,
        username: String::from("aa"),
    };

    TResponse {
        code: 0,
        data: user,
    }
}

