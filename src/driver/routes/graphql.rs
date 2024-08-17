//use std::sync::Arc;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Object, Request, Response, Schema,
};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension, Json,
};

//use crate::driver::modules::Modules;
use crate::adapter::{persistence::mysql::Db, repository::graphql::BankQueryRepository};
use crate::app::usecase::graphql::BankQuery;
use crate::domain::model::graphql::Ping;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn ping(&self) -> Ping {
        let db = Db::new().await;
        let bank_query = BankQuery::new(BankQueryRepository::new(db.clone()));

        bank_query.ping().await
    }
}

pub type BankSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub async fn graphql_handler(
    schema: Extension<BankSchema>,
    //Extension(modules): Extension<Arc<Modules>>,
    req: Json<Request>,
) -> Json<Response> {
    schema.execute(req.0).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

pub async fn not_found_handler() -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, "not found".to_string())
}
