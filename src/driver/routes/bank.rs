use axum::{
    extract::Path,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    body::Body,
    Extension,
    Json,
};
use std::{
    io::Cursor,
    sync::Arc
};
use tokio_util::io::ReaderStream;
use serde_json::json;

use crate::driver::{
    model::bank::{
        JsonAccountView, JsonCreateAccount, JsonCreateHistory, JsonHistoriesView, JsonHistoriesDownload, JsonUpdateMoney, JsonLogin,
    },
    modules::{Modules, ModulesExt},
};

pub async fn find_account(
    Extension(modules): Extension<Arc<Modules>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.bank_manager_use_case().view_account(id).await;

    match res {
        Ok(account) => account
            .map(|data| {
                let json: JsonAccountView = data.into();

                (StatusCode::OK, Json(json))
            })
            .ok_or_else(|| StatusCode::NOT_FOUND),
        Err(_) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn find_histories(
    Extension(modules): Extension<Arc<Modules>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.bank_manager_use_case().view_histories(id).await;

    match res {
        Ok(histories) => histories
            .map(|data| {
                let json: JsonHistoriesView = data.into();

                (StatusCode::OK, Json(json))
            })
            .ok_or_else(|| StatusCode::NOT_FOUND),
        Err(_) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn download_histories(
    Extension(modules): Extension<Arc<Modules>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.bank_manager_use_case().download_histories(id).await;

    match res {
        Ok(dl_histories) => dl_histories
            .map(|data| {
                let json: Vec<JsonHistoriesDownload> = data.into_iter().map(|d| d.into()).collect();
                let json_data = serde_json::to_vec(&json).expect("error");
                let cursor = Cursor::new(json_data);

                let stream = ReaderStream::new(cursor);
                let body = Body::from_stream(stream);

                let mut headers = HeaderMap::new();
                headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
                headers.insert(
                    header::CONTENT_DISPOSITION,
                    HeaderValue::from_static("attachment; filename=\"deposit_histories.json\""),
                );

                (headers, body).into_response()
            })
            .ok_or_else(|| StatusCode::NOT_FOUND),
        Err(_) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_account(
    Extension(modules): Extension<Arc<Modules>>,
    Json(params): Json<JsonCreateAccount>,
) -> Result<impl IntoResponse, StatusCode> {
    modules
        .bank_manager_use_case()
        .add_account(params.into())
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| {
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn create_history(
    Extension(modules): Extension<Arc<Modules>>,
    Json(params): Json<JsonCreateHistory>,
) -> Result<impl IntoResponse, StatusCode> {
    modules
        .bank_manager_use_case()
        .add_history(params.into())
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| {
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn update_money(
    Extension(modules): Extension<Arc<Modules>>,
    Path(id): Path<String>,
    Json(params): Json<JsonUpdateMoney>,
) -> Result<impl IntoResponse, StatusCode> {
    modules
        .bank_manager_use_case()
        .manage_money(id, params.into())
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| {
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn login_account(
    Extension(modules): Extension<Arc<Modules>>,
    Json(params): Json<JsonLogin>,
) -> Result<impl IntoResponse, StatusCode> {
//) -> Result<Json<serde_json::Value>, StatusCode> {
    /*modules
        .bank_manager_use_case()
        .signup_account(params.into())
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| {
            StatusCode::UNAUTHORIZED
        })
    */

    let res = modules
        .bank_manager_use_case()
        .signup_account(params.into())
        .await;
    /*
    match res {
        Ok(account) => {
            let pared_hash = PasswordHash::new(&account.password).map_err(|_| anyhow::Error::msg("Password parse error"))?;
            Argon2::default().verify_password(data.password.as_bytes(), &pared_hash)
            .map(|_| StatusCode::OK)
            .map_err(|_| StatusCode::UNAUTHORIZED)
        }
        Err(_) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
    */

    match res {
        Ok(true) => {
            Ok(Json(json!({ "status": "ok" })))
        },
        Ok(false) => {
            Ok(Json(json!({ "status": "err" })))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}