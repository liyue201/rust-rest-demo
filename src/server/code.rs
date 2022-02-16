use std::collections::HashMap;

use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use lazy_static::lazy_static;
use serde::Serialize;
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
    pub static ref TCODE_MESSAGE: HashMap<TCode, &'static str> = {
        let mut m = HashMap::new();
        m.insert(TCode::Ok, "Ok");
        m.insert(TCode::UnknownError, "Unknown error");
        m.insert(TCode::UsernameNotExist, "Username not exist");
        m.insert(TCode::PasswordError, "Password error");
        m.insert(TCode::DbError, "Db error");
        m
    };
}

#[derive(Serialize)]
pub struct ComResponse<T: Serialize> {
    pub code: TCode,
    pub msg: Option<String>,
    pub data: Option<T>,
}

#[derive(Debug)]
pub enum TResponse<T: Serialize> {
    Ok(T),
    Err(TCode, String),
}

impl<T: Serialize> IntoResponse for TResponse<T>
{
    fn into_response(self) -> Response {
        let resp = match self {
            TResponse::Ok(data) => {
                ComResponse {
                    code: TCode::Ok,
                    msg: Some(TCODE_MESSAGE.get(&TCode::Ok).unwrap().to_string()),
                    data: Some(data),
                }
            }
            TResponse::Err(code, msg) => {
                ComResponse {
                    code,
                    msg: Some(msg),
                    data: None,
                }
            }
        };
        let body = Json(json!(resp));
        (StatusCode::from_u16(200).unwrap(), body).into_response()
    }
}