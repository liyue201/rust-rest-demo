use result::Result;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::result;
use std::sync::Arc;

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

#[derive(Debug)]
pub enum TError {
    Error(TCode, String),
}

#[derive(Serialize)]
pub struct TResponse<T> {
    pub code: TCode,
    pub msg: Option<String>,
    pub data: Option<T>,
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