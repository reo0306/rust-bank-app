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
}
