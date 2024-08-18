use anyhow::Result;
use async_trait::async_trait;

use crate::domain::model::{
    Id,
    graphql::{Ping, BankQueryAccount}
};

#[async_trait]
pub trait BankQueryRepository {
    async fn ping(&self) -> Result<Ping>;
    async fn find_account(&self, id: &Id<BankQueryAccount>) -> Result<Option<BankQueryAccount>>;
}
