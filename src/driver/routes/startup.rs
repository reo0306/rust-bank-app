use anyhow::Result;
use std::sync::Arc;
use axum::{
    routing::{get, patch, post}, Extension, Router
};

use crate::driver::{
    modules::Modules,
    routes::bank::{
        find_account,
        create_account,
        update_money,
        find_histories,
        create_history,
    },
};

pub async fn run(modules: Arc<Modules>) -> Result<()> {
    let bank_router = Router::new()
        .route("/", post(create_account))
        .route("/:id", get(find_account))
        .route("/history/:id", get(find_histories))
        .route("/history/:id/:money", post(create_history))
        .route("/money/:id/:money", patch(update_money));

    let app = Router::new()
        .nest("/bank", bank_router)
        .layer(Extension(modules));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("Server cannnot launch!"));

    Ok(())
}