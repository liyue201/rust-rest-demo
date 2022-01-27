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
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use serde::{Deserialize, Serialize};
use crate::server::code::*;


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

