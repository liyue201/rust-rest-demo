use std::result;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize)]
pub struct User {
    id: i32,
    username: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginReq {
    username: String,
    password: String,
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

pub async fn login(Json(req): Json<LoginReq>) -> impl IntoResponse {
    info!("login req{:?}", req);

    let user = User {
        id: 10,
        username: req.username,
    };

    TResponse {
        code: 0,
        data: user,
    }
}

