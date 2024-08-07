use async_trait::async_trait;
use anyhow::Result;

use super::DatabaseRepositoryImpl;
use crate::domain::{
    model::{
        Id,
        bank::{BankAccount, DepositHistories, NewBankAccount, NewDepositHistory, RenewMoney}
    },
    repository::bank::BankManagerRepository,
};
use crate::adapter::model::bank::{
    BankAccountTable,
    DepositHistoriesTable,
    NewBankAccountRecord,
    NewDepositHistoryRecord,
    RenewMoneyRecord,
};

#[async_trait]
impl BankManagerRepository for DatabaseRepositoryImpl<BankAccount> {
    async fn find_account(&self, id: &Id<BankAccount>) -> Result<Option<BankAccount>> {
        let pool = self.pool.0.clone();

        let bank_account_table = sqlx::query_as::<_, BankAccountTable>(
            r#"
            SELECT bank_id, branch_office_id, name, money FROM bank_accounts WHERE id = ?;
            "#
        )
        .bind(id.value.to_string())
        .fetch_one(&*pool)
        .await
        .ok();

        bank_account_table
            .map_or(Ok(None), |data| Ok(Some(data.try_into()?)))
    }

    async fn find_histories(&self, id: &Id<DepositHistories>) -> Result<Option<DepositHistories>> {
        let pool = self.pool.0.clone();

        let deposit_histories_table = sqlx::query_as::<_, DepositHistoriesTable>(
            r#"
            SELECT bank_account_id, action, money FROM deposit_histories WHERE id = ?;
            "#
        )
        .bind(id.value.to_string())
        .fetch_one(&*pool)
        .await
        .ok();

        deposit_histories_table
            .map_or(Ok(None), |data| Ok(Some(data.try_into()?)))
    }

    async fn create_new_account(&self, params: NewBankAccount) -> Result<()> {
        let pool = self.pool.0.clone();

        let new_bank_account_record: NewBankAccountRecord = params.try_into()?;

        sqlx::query(
            r#"
            INSERT INTO bank_accounts (id, bank_id, branch_office_id, name, money) VALUES(?, ?, ?, ?, ?);
            "#,
        )
        .bind(new_bank_account_record.id)
        .bind(new_bank_account_record.bank_id)
        .bind(new_bank_account_record.branch_office_id)
        .bind(new_bank_account_record.name)
        .bind(new_bank_account_record.money)
        .execute(&*pool)
        .await?;

        Ok(())
    }

    async fn create_new_history(&self, params: NewDepositHistory) -> Result<()> {
        let pool = self.pool.0.clone();

        let new_deposit_history_record: NewDepositHistoryRecord = params.try_into()?;

        sqlx::query(
            r#"
            INSERT INTO deposit_histories (id, bank_account_id, action, money) VALUES(?, ?, ?, ?);
            "#,
        )
        .bind(new_deposit_history_record.id)
        .bind(new_deposit_history_record.bank_account_id)
        .bind(new_deposit_history_record.action)
        .bind(new_deposit_history_record.money)
        .execute(&*pool)
        .await?;

        Ok(())
    }

    async fn update_money(&self, id: &Id<BankAccount>, params: RenewMoney) -> Result<()> {
        let pool = self.pool.0.clone();

        let renew_money_record: RenewMoneyRecord = params.try_into()?;

        sqlx::query(
            r#"
            UPDATE bank_accounts SET money = ? WHERE id = ?;
            "#,
        )
        .bind(renew_money_record.money)
        .bind(id.value.to_string())
        .execute(&*pool)
        .await?;

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