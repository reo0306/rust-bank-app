use std::sync::Arc;
use async_graphql::{Object, Context};

use crate::driver::{
    modules::{Modules, ModulesExt},
    model::graphql::CreateAccount,
};
use crate::domain::model::graphql::{Ping, BankQueryAccount};

pub struct QueryRoot;
pub struct MutationRoot;

#[Object]
impl QueryRoot {
    async fn ping(&self, ctx: &Context<'_>) -> Ping {
        ctx.data::<Arc<Modules>>().unwrap().bank_query_use_case().ping().await
    }

    async fn find(&self, ctx: &Context<'_>, id: String) -> BankQueryAccount {
        let res = ctx.data::<Arc<Modules>>()
            .unwrap()
            .bank_query_use_case()
            .view_account(id)
            .await;

        match res {
            Ok(account) => account.unwrap(),
            Err(e) => panic!("error: {}", e),
        }
    }
}

#[Object]
impl MutationRoot {
    async fn create(&self, ctx: &Context<'_>, create_account: CreateAccount) -> BankQueryAccount {
        let res = ctx.data::<Arc<Modules>>()
            .unwrap()
            .bank_query_use_case()
            .add_account(create_account.into())
            .await;

        match res {
            Ok(new_account) => new_account.unwrap(),
            Err(e) => panic!("error: {}", e),
        }
    }
}