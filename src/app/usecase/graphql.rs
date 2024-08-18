use anyhow::Result;
use derive_new::new;
use std::sync::Arc;

use crate::adapter::module::RepositoriesModuleExt;
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
}

/*
use crate::adapter::repository::graphql::BankQueryRepository;
use crate::domain::model::graphql::{Ping, BankQueryAccount};

pub struct BankQuery {
    repository: BankQueryRepository,
}

impl BankQuery {
    pub fn new(repository: BankQueryRepository) -> Self {
        Self { repository }
    }

    pub async fn ping(&self) -> Ping {
        self.repository.ping().await.unwrap()
    }

    pub async fn view_accounts(&self) -> BankAccount {
        self.repositories
            .find_account()
            .await
            .map(|account| account.map(|a| a.into()))
    }
}*/
