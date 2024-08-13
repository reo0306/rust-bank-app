use anyhow::Result;
use axum::{
    routing::{get, patch, post},
    Extension, Router,
};
use std::sync::Arc;

use crate::driver::{
    modules::Modules,
    routes::bank::{create_account, create_history, find_account, find_histories, update_money},
    //routes::deposit_history::{find_dynamodb_histories, create_dynamodb_history},
    routes::deposit_history::create_dynamodb_history,
};

pub async fn run(modules: Arc<Modules>) -> Result<()> {
    let bank_router = Router::new()
        .route("/", post(create_account))
        .route("/:id", get(find_account))
        .route("/history", post(create_history))
        .route("/history/:id", get(find_histories))
        .route("/money/:id", patch(update_money));

    let bank_dynamodb_router = Router::new()
        .route("/history", post(create_dynamodb_history));
        //.route("/history/:id", get(find_dynamodb_histories));

    let app = Router::new()
        .nest("/bank", bank_router)
        .nest("/bank/dynamodb", bank_dynamodb_router)
        .layer(Extension(modules));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("Server cannnot launch!"));

    Ok(())
}
