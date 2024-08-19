use anyhow::Result;
use derive_new::new;
use std::sync::Arc;

use crate::adapter::module::RepositoriesModuleExt;
use crate::app::model::graphql::CreateBankMutationAccount;
use crate::domain::{
  model::graphql::{Ping, BankQueryAccount},
  repository::graphql::BankQueryRepository,
};

#[derive(new)]
pub struct BankQueryUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> BankQueryUseCase<R> {
  pub async fn ping(&self) -> Ping {
    self.repositories.bank_query_repository().ping().await.unwrap()
  }

  pub async fn view_account(&self, id: String) -> Result<Option<BankQueryAccount>> {
    self.repositories
        .bank_query_repository()
        .find_account(&id.try_into()?)
        .await
        .map(|account| account.map(|a| a.into()))
  }

  //pub async fn add_account(&self, data: CreateBankMutationAccount) -> Result<()> {
  pub async fn add_account(&self, data: CreateBankMutationAccount) -> Result<Option<BankQueryAccount>> {
    self.repositories
        .bank_query_repository()
        .create_new_account(data.try_into()?)
        .await
        .map(|account| account.map(|a| a.into()))
  }
}