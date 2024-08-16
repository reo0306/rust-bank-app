use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
    Json,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation,
    EmptySubscription,
    Request,
    Response,
    Schema,
};

use crate::adapter::repository::graphql::QueryRoot;

pub type BankSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub async fn graphql_handler(schema: Extension<BankSchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

pub async fn not_found_handler() -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, "not found".to_string())
}
