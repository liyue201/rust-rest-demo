use std::sync::Arc;

use axum::extract::Extension;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use super::code::*;
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

pub async fn register(store: Extension<Arc<Store>>, Json(req): Json<RegisterReq>) -> impl IntoResponse {
    info!("register req{:?}", req);
    let user = User {
        id: 0,
        username: Some(req.username),
        password: Some(req.password),
    };

    let r = store.create_user(user).await;

    r.map(|u| {
        TSuccess {
            data: UserData {
                id: u.id,
                username: u.username.unwrap().clone(),
            }
        }
    }).map_err(|err| {
        error!("failed to create user:{}", err);
        TError { code: TCode::DbError, msg: Some("db error".to_owned()) }
    })
}

pub async fn login(store: Extension<Arc<Store>>, Json(req): Json<LoginReq>) -> impl IntoResponse {
    info!("login req{:?}", req);

    let r = store.fetch_user_by_name(req.username.as_str()).await;

    r.map(|res| {
        res.map(|u| {
            TSuccess {
                data: UserData {
                    id: u.id,
                    username: u.username.unwrap().clone(),
                }
            }
        }).ok_or(TError {
            code: TCode::UsernameNotExist,
            msg: Some(TCODE_MESSAGE.get(&TCode::UsernameNotExist).unwrap().to_string()),
        })
    }).map_err(|err| {
        error!("failed to get user:{}", err);
        TError { code: TCode::DbError, msg: Some("db error".to_owned()) }
    })
}

