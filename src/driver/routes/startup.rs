use anyhow::Result;
use axum::{
    routing::{get, patch, post},
    Extension, Router,
};
use std::sync::Arc;

use crate::driver::{
    modules::Modules,
    routes::bank::{create_account, create_history, find_account, find_histories, update_money},
};

pub async fn run(modules: Arc<Modules>) -> Result<()> {
    let bank_router = Router::new()
        .route("/", post(create_account))
        .route("/:id", get(find_account))
        .route("/history", post(create_history))
        .route("/history/:id", get(find_histories))
        .route("/money/:id", patch(update_money));

    let app = Router::new()
        .nest("/bank", bank_router)
        .layer(Extension(modules));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("Server cannnot launch!"));

    Ok(())
}
