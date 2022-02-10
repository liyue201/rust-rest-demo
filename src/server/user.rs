use std::sync::Arc;

use axum::extract::Extension;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::server::code::*;
use crate::store::Store;
use crate::store::user::User;

#[derive(Debug, Serialize)]
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

pub async fn register(store: Extension<Arc<Store>>, Json(req): Json<RegisterReq>) -> TResponse<UserData> {
    info!("register req{:?}", req);
    let user = User {
        id: 0,
        username: Some(req.username),
        password: Some(req.password),
    };

    let r = store.create_user(user).await;

    return match r {
        Ok(u) => {
            TResponse::Ok(UserData {
                id: u.id,
                username: u.username.unwrap().clone(),
            })
        }
        Err(err) => {
            error!("failed to create user:{}", err);
            TResponse::Err(TCode::DbError, "db error".to_owned())
        }
    };
}

pub async fn login(store: Extension<Arc<Store>>, Json(req): Json<LoginReq>) -> TResponse<UserData> {
    info!("login req{:?}", req);

    let r = store.fetch_user_by_name(req.username.as_str()).await;

    return match r {
        Ok(res) => {
            match res {
                Some(user) => {
                    TResponse::Ok(UserData {
                        id: user.id,
                        username: user.username.unwrap().clone(),
                    })
                }
                None => {
                    TResponse::Err(TCode::UsernameNotExist, TCODE_MESSAGE.get(&TCode::UsernameNotExist).unwrap().to_string())
                }
            }
        }
        Err(err) => {
            error!("failed to get user:{}", err);
            TResponse::Err(TCode::DbError, "db error".to_owned())
        }
    };
}

