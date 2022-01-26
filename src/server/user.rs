use std::ops::Deref;
use std::result;
use std::sync::Arc;

use axum::extract::Extension;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[crud_table]
#[derive(Clone, Debug)]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
}

#[derive(Serialize)]
pub struct UserData {
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
    data: Option<T>,
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

pub async fn login(rb: Extension<Arc<Rbatis>>, Json(req): Json<LoginReq>) -> impl IntoResponse {
    info!("login req{:?}", req);

    let r = rb.fetch_by_column::<Option<User>, _>("username", req.username.clone()).await;

    return match r {
        Ok(res) => {
            match res {
                Some(user) => {
                    TResponse {
                        code: 0,
                        data: Some(UserData {
                            id: user.id,
                            username: user.username.unwrap().clone(),
                        }),
                    }
                }
                None => {
                    TResponse {
                        code: 300,
                        data: None,
                    }
                }
            }
        }
        Err(err) => {
            error!("failed to get user:{}", err);
            TResponse {
                code: 100,
                data: None,
            }
        }
    };
}

