use anyhow::Result;
use async_graphql::{EmptySubscription, Schema};
use axum::{
    routing::{get, patch, post},
    Extension, Router,
};
use std::sync::Arc;

use crate::driver::routes::resolver::{QueryRoot, MutationRoot};
use crate::driver::{
    modules::Modules,
    routes::bank::{create_account, create_history, find_account, find_histories, download_histories, update_money, login_account},
    routes::deposit_history::{create_dynamodb_history, find_dynamodb_history},
    routes::graphql::{graphql_handler, graphql_playground, not_found_handler},
};

pub async fn run(modules: Arc<Modules>) -> Result<()> {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(modules.clone())
        .finish();

    let bank_router = Router::new()
        .route("/", post(create_account))
        .route("/:id", get(find_account))
        .route("/login", post(login_account))
        .route("/history", post(create_history))
        .route("/history/:id", get(find_histories))
        .route("/history/:id/download", get(download_histories))
        .route("/money/:id", patch(update_money));

    let bank_dynamodb_router = Router::new()
        .route("/history", post(create_dynamodb_history))
        .route("/history/:id", get(find_dynamodb_history));

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .nest("/bank", bank_router)
        .nest("/bank/dynamodb", bank_dynamodb_router)
        .layer(Extension(schema))
        .layer(Extension(modules))
        .fallback(not_found_handler);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("Server cannnot launch!"));

    Ok(())
}
