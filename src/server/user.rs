use result::Result;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::result;
use std::sync::Arc;

use axum::body::boxed;
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use lazy_static::lazy_static;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_repr::Serialize_repr;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize_repr)]
#[repr(u8)]
pub enum TCode {
    Ok = 0,
    UnknownError = 1,
    UsernameNotExist = 2,
    PasswordError = 3,
    DbError = 4,
}

lazy_static! {
    static ref TCODE_MESSAGE: HashMap<TCode, &'static str> = {
        let mut m = HashMap::new();
        m.insert(TCode::Ok, "Ok");
        m.insert(TCode::UnknownError, "Unknown error");
        m.insert(TCode::UsernameNotExist, "Username not exist");
        m.insert(TCode::PasswordError, "Password error");
        m.insert(TCode::DbError, "Db error");
        m
    };
}

#[derive(Debug)]
pub enum TError {
    Error(TCode, String),
}

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
    code: TCode,
    msg: Option<String>,
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

impl IntoResponse for TError {
    fn into_response(self) -> Response {
        let mut resp = TResponse {
            code: TCode::UnknownError,
            msg: Some(TCODE_MESSAGE.get(&TCode::UnknownError).unwrap().to_string()),
            data: Some(()),
        };
        match self {
            TError::Error(code, msg) => {
                resp.code = code;
                resp.msg = Some(msg);
            }
        }
        let body = Json(json!(resp));
        (StatusCode::from_u16(200).unwrap(), body).into_response()
    }
}

pub async fn login(rb: Extension<Arc<Rbatis>>, Json(req): Json<LoginReq>) -> Result<impl IntoResponse, TError> {
    info!("login req{:?}", req);

    let r = rb.fetch_by_column::<Option<User>, _>("username", req.username.clone()).await;

    return match r {
        Ok(res) => {
            match res {
                Some(user) => {
                    Ok(TResponse {
                        code: TCode::Ok,
                        msg: None,
                        data: Some(UserData {
                            id: user.id,
                            username: user.username.unwrap().clone(),
                        }),
                    })
                }
                None => {
                    Err(TError::Error(TCode::UsernameNotExist, TCODE_MESSAGE.get(&TCode::UsernameNotExist).unwrap().to_string()))
                }
            }
        }
        Err(err) => {
            error!("failed to get user:{}", err);
            Err(TError::Error(TCode::DbError, "db error".to_owned()))
        }
    };
}

