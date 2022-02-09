use result::Result;
use std::result;
use std::sync::Arc;

use axum::extract::Extension;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::server::code::*;
use crate::store::Store;
use crate::store::user::User;

#[derive(Serialize)]
pub struct UserData {
    id: i64,
    username: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterReq {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginReq {
    username: String,
    password: String,
}

pub async fn register(store: Extension<Arc<Store>>, Json(req): Json<RegisterReq>) -> Result<impl IntoResponse, TError> {
    info!("register req{:?}", req);
    let user = User {
        id: 0,
        username: Some(req.username),
        password: Some(req.password),
    };

    let r = store.create_user(user).await;

    return match r {
        Ok(u) => {
            Ok(TResponse {
                code: TCode::Ok,
                msg: None,
                data: Some(UserData {
                    id: u.id,
                    username: u.username.unwrap().clone(),
                }),
            })
        }
        Err(err) => {
            error!("failed to create user:{}", err);
            Err(TError::Error(TCode::DbError, "db error".to_owned()))
        }
    };
}

pub async fn login(store: Extension<Arc<Store>>, Json(req): Json<LoginReq>) -> Result<impl IntoResponse, TError> {
    info!("login req{:?}", req);

    let r = store.fetch_user_by_name(req.username.as_str()).await;

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

