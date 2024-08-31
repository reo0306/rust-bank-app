use anyhow::Result;
use async_trait::async_trait;

use crate::domain::model::{
    bank::{
        BankAccount, DepositHistories, NewBankAccount, NewDepositHistory, DepositDownloadHistories, RenewMoney, SignupBankAccount,
    },
    Id,
};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait BankManagerRepository {
    async fn find_account(&self, id: &Id<BankAccount>) -> Result<Option<BankAccount>>;
    async fn find_histories(&self, id: &Id<DepositHistories>) -> Result<Option<DepositHistories>>;
    async fn create_new_account(&self, params: NewBankAccount) -> Result<()>;
    async fn create_new_history(&self, params: NewDepositHistory) -> Result<()>;
    async fn find_download_histories(&self, id: &Id<DepositHistories>) -> Result<Option<Vec<DepositDownloadHistories>>>;
    async fn update_money(&self, id: &Id<BankAccount>, params: RenewMoney) -> Result<()>;
    async fn find_login_account(&self, id: &Id<BankAccount>) -> Result<Option<SignupBankAccount>>;
}
