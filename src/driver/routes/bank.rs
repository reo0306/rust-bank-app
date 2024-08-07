use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Extension,
    Json
};
use std::sync::Arc;

use crate::driver::{
    model::bank::{
        JsonAccountView,
        JsonHistoriesView,
        JsonCreateAccount,
        JsonCreateHistory,
        JsonUpdateMoney,
    },
    modules::{Modules, ModulesExt},
};

pub async fn find_account(
    Extension(modules): Extension<Arc<Modules>>,
    Path(id): Path<String>,
    ) -> Result<impl IntoResponse, StatusCode> {
        let res = modules.bank_manager_use_case().view_account(id).await;

        match res {
            Ok(account) => account.map(|data| {
                let json: JsonAccountView = data.into();

                (StatusCode::OK, Json(json))
            })
            .ok_or_else(|| StatusCode::NOT_FOUND),
            Err(_) => {
                //error!("Find account error: {:?}", err);
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
            Ok(histories) => histories.map(|data| {
                let json: JsonHistoriesView = data.into();

                (StatusCode::OK, Json(json))
            })
            .ok_or_else(|| StatusCode::NOT_FOUND),
            Err(_) => {
                //error!("Find history error: {:?}", err);
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
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(|_| {
                //error!("Create account error: {}", err);
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
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(|_| {
                //error!("Create history error: {}", err);
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
                //error!("Update money error: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })
}
