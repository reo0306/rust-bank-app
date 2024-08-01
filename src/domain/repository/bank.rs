use async_trait::async_trait;
use anyhow::Result;

use crate::domain::model::bank::{BankAccount, AccountHistories};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait BankRepository {
    async fn create_new_account(&self, user_id: &String) -> Result<()>;
    async fn find_account(&self, user_id: &String) -> Result<BankAccount>;
    async fn histories(&self, user_id: &String) -> Result<Option<AccountHistories>>;
    async fn payment(&self, user_id: &String, money: i32) -> Result<()>;
    async fn debit(&self, user_id: &String, money: i32) -> Result<()>;
}