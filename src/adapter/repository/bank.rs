use async_trait::async_trait;
use anyhow::Result;

use crate::domain::{
    model::bank::{BankAccount, AccountHistories},
    repository::bank::BankManagerRepository,
};

pub struct BankManager;

#[async_trait]
impl BankManagerRepository for BankManager {
    async fn find_account(&self, id: &Id<BankAccount>) -> Result<Option<BankAccount>> {
        println!("user: {}", id);
        Ok(BankAccount)
    }

    async fn find_histories(&self, id: &Id<DepositHistories>) -> Result<Option<AccountHistories>> {
        println!("user: {}", id);
        Ok(Some(AccountHistories))
    }

    async fn create_new_account(&self, id: &Id<BankAccount>, params: NewBankAccount) -> Result<()> {
        println!("user: {}", id);
        Ok(())
    }

    async fn create_new_history(&self, id: &Id<DepositHistories>, params: NewDepositHistory) -> Result<()> {
        println!("user: {}", id);
        Ok(())
    }

    async fn update_money(&self, id: &Id<BankAccount>, params: RenewMoney) -> Result<()> {
        println!("user: {}, money: {}", user_id, money);
        Ok(())
    }
}

#[cfg(test)]
mod bank_test {
    use mockall::predicate::*;

    use crate::domain::{
        model::bank::{BankAccount, AccountHistories},
        repository::bank::{BankManagerRepository, MockBankManagerRepository},
    };

    #[tokio::test]
    async fn new_accounts_can_be_created() {
        let mut sut = MockBankManagerRepository::new();

        let user_id = "user123".to_string();

        sut.expect_create_new_account()
            .with(eq(user_id.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let result = sut.create_new_account(&user_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn account_can_be_found() {
        let mut sut = MockBankManagerRepository::new();

        let user_id = "user123".to_string();

        sut.expect_find_account()
            .with(eq(user_id.clone()))
            .times(1)
            .returning(|_| Ok(BankAccount));

        let result = sut.find_account(&user_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn histories_can_be_found() {
        let mut sut = MockBankManagerRepository::new();

        let user_id = "user123".to_string();

        sut.expect_histories()
            .with(eq(user_id.clone()))
            .times(1)
            .returning(|_| Ok(Some(AccountHistories)));

        let result = sut.histories(&user_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn payment_can_be_made() {
        let mut sut = MockBankManagerRepository::new();

        let user_id = "user123".to_string();
        let money: i32 = 10;

        sut.expect_payment()
            .with(eq(user_id.clone()), eq(money))
            .times(1)
            .returning(|_, _| Ok(()));

        let result = sut.payment(&user_id, money).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn debit_can_be_made() {
        let mut sut = MockBankManagerRepository::new();

        let user_id = "user123".to_string();
        let money: i32 = 10;

        sut.expect_debit()
            .with(eq(user_id.clone()), eq(money))
            .times(1)
            .returning(|_, _| Ok(()));

        let result = sut.debit(&user_id, money).await;

        assert!(result.is_ok());
    }
}