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
pub enum Code {
    Ok = 0,
    UnknownError = 1,
    UsernameNotExist = 2,
    PasswordError = 3,
    DbError = 4,
}

lazy_static! {
    pub static ref TCODE_MESSAGE: HashMap<Code, &'static str> = {
        let mut m = HashMap::new();
        m.insert(Code::Ok, "Ok");
        m.insert(Code::UnknownError, "Unknown error");
        m.insert(Code::UsernameNotExist, "Username not exist");
        m.insert(Code::PasswordError, "Password error");
        m.insert(Code::DbError, "Db error");
        m
    };
}

#[derive(Serialize)]
pub struct ComResponse<T: Serialize> {
    pub code: Code,
    pub msg: Option<String>,
    pub data: Option<T>,
}

#[derive(Debug)]
pub struct Success<T: Serialize> {
    pub data: T,
}

impl<T: Serialize> IntoResponse for Success<T>
{
    fn into_response(self) -> Response {
        let resp = ComResponse {
            code: Code::Ok,
            msg: Some(TCODE_MESSAGE.get(&Code::Ok).unwrap().to_string()),
            data: Some(self.data),
        };
        let body = Json(json!(resp));
        (StatusCode::from_u16(200).unwrap(), body).into_response()
    }
}

#[derive(Debug)]
pub struct Error {
    pub code: Code,
    pub msg: Option<String>,
}

impl IntoResponse for Error
{
    fn into_response(self) -> Response {
        let resp: ComResponse<i32> = {
            ComResponse {
                code: self.code,
                msg: self.msg,
                data: None,
            }
        };
        let body = Json(json!(resp));
        (StatusCode::from_u16(200).unwrap(), body).into_response()
    }
}
