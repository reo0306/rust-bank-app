use async_trait::async_trait;
use anyhow::Result;

use crate::domain::model::{
    Id,
    bank::{BankAccount, DepositHistories,NewBankAccount, NewDepositHistory, RenewMoney},
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait BankManagerRepository {
    async fn find_account(&self, id: &Id<BankAccount>) -> Result<Option<BankAccount>>;
    async fn find_histories(&self, id: &Id<DepositHistories>) -> Result<Option<DepositHistories>>;
    async fn create_new_account(&self, id: &Id<BankAccount>, params: NewBankAccount) -> Result<()>;
    async fn create_new_history(&self, id: &Id<DepositHistories>, params: NewDepositHistory) -> Result<()>;
    async fn update_money(&self, id: &Id<BankAccount>, params: RenewMoney) -> Result<()>;
}