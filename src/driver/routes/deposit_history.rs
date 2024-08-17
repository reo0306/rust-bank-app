use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use std::sync::Arc;

use crate::driver::{
    model::bank::{JsonCreateHistory, JsonHistoriesView},
    modules::{Modules, ModulesExt},
};

pub async fn find_dynamodb_history(
    Extension(modules): Extension<Arc<Modules>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = modules.deposit_history_use_case().view_history(id).await;

    match res {
        Ok(history) => history
            .map(|data| {
                let json: JsonHistoriesView = data.into();

                (StatusCode::OK, Json(json))
            })
            .ok_or_else(|| StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_dynamodb_history(
    Extension(modules): Extension<Arc<Modules>>,
    Json(params): Json<JsonCreateHistory>,
) -> Result<impl IntoResponse, StatusCode> {
    modules
        .deposit_history_use_case()
        .add_history(params.into())
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
