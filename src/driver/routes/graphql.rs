use std::sync::Arc;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Object, Request, Response, Schema, Context,
};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension, Json,
};

use crate::driver::modules::{Modules, ModulesExt};
use crate::domain::model::graphql::{Ping, BankQueryAccount};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn ping(&self, ctx: &Context<'_>) -> Ping {
        let modules = ctx.data_unchecked::<Arc<Modules>>();
        modules.bank_query_use_case().ping().await
    }

    async fn find(&self, ctx: &Context<'_>, id: String) -> BankQueryAccount {
        let modules = ctx.data_unchecked::<Arc<Modules>>();
        let res = modules.bank_query_use_case().view_account(id).await;

        match res {
            Ok(account) => account.unwrap(),
            Err(e) => panic!("error: {}", e),
        }
    }
}

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
