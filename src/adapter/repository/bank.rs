use anyhow::Result;
use async_trait::async_trait;

use super::DatabaseRepositoryImpl;
use crate::adapter::model::bank::{
    BankAccountTable, DepositHistoriesTable, NewBankAccountRecord, NewDepositHistoryRecord,
    RenewMoneyRecord,
};
use crate::domain::{
    model::{
        bank::{BankAccount, DepositHistories, NewBankAccount, NewDepositHistory, DepositDownloadHistories, RenewMoney},
        Id,
    },
    repository::bank::BankManagerRepository,
};

#[async_trait]
impl BankManagerRepository for DatabaseRepositoryImpl<BankAccount> {
    async fn find_account(&self, id: &Id<BankAccount>) -> Result<Option<BankAccount>> {
        let pool = self.pool.0.clone();

        let bank_account_table = sqlx::query_as::<_, BankAccountTable>(
            r#"
            SELECT bank_id, branch_office_id, name, money FROM bank_accounts WHERE id = ?;
            "#,
        )
        .bind(id.value.to_string())
        .fetch_one(&*pool)
        .await
        .ok();

        bank_account_table.map_or(Ok(None), |data| Ok(Some(data.try_into()?)))
    }

    async fn find_histories(&self, id: &Id<DepositHistories>) -> Result<Option<DepositHistories>> {
        let pool = self.pool.0.clone();

        let deposit_histories_table = sqlx::query_as::<_, DepositHistoriesTable>(
            r#"
            SELECT bank_account_id, action, money FROM deposit_histories WHERE id = ?;
            "#,
        )
        .bind(id.value.to_string())
        .fetch_one(&*pool)
        .await
        .ok();

        deposit_histories_table.map_or(Ok(None), |data| Ok(Some(data.try_into()?)))
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

    async fn find_download_histories(&self, id: &Id<DepositHistories>) -> Result<Option<Vec<DepositDownloadHistories>>> {
        let pool = self.pool.0.clone();

        let deposit_histories_table = sqlx::query_as::<_, DepositHistoriesTable>(
            r#"
            SELECT id, bank_account_id, action, money FROM deposit_histories WHERE bank_account_id = ?;
            "#,
        )
        .bind(id.value.to_string())
        .fetch_all(&*pool)
        .await
        .ok();

        deposit_histories_table.map_or(
            Ok(Some(vec![DepositDownloadHistories::new()])),
            |data| Ok(Some(data.try_into()?))
        )
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
mod bank_repository_unit_test {
    use mockall::predicate::*;
    use ulid::Ulid;

    use crate::domain::{
        model::{
            bank::{BankAccount, DepositHistories, NewBankAccount, NewDepositHistory, RenewMoney},
            Id,
        },
        repository::bank::{BankManagerRepository, MockBankManagerRepository},
    };

    #[tokio::test]
    async fn new_account_can_be_created() {
        let mut sut = MockBankManagerRepository::new();

        let data = NewBankAccount {
            id: Id::new(Ulid::new()),
            bank_id: "1".to_string(),
            branch_office_id: "2".to_string(),
            name: "test".to_string(),
            money: 0,
        };

        sut.expect_create_new_account()
            .with(eq(data.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let result = sut.create_new_account(data).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn account_can_be_found() {
        let mut sut = MockBankManagerRepository::new();

        let id = Id::new(Ulid::new());

        sut.expect_find_account()
            .with(eq(id.clone()))
            .times(1)
            .returning(|_| {
                Ok(Some(BankAccount {
                    bank_id: "a".to_string(),
                    branch_office_id: "b".to_string(),
                    name: "c".to_string(),
                    money: 1,
                }))
            });

        let result = sut.find_account(&id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn new_deposit_history_can_be_created() {
        let mut sut = MockBankManagerRepository::new();

        let data = NewDepositHistory {
            id: Id::new(Ulid::new()),
            bank_account_id: "1".to_string(),
            action: "payment".to_string(),
            money: 100,
        };

        sut.expect_create_new_history()
            .with(eq(data.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let result = sut.create_new_history(data).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn deposit_histories_can_be_found() {
        let mut sut = MockBankManagerRepository::new();

        let id = Id::new(Ulid::new());

        sut.expect_find_histories()
            .with(eq(id.clone()))
            .times(1)
            .returning(|_| {
                Ok(Some(DepositHistories {
                    bank_account_id: "1".to_string(),
                    action: "payment".to_string(),
                    money: 100,
                }))
            });

        let result = sut.find_histories(&id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn renew_money_can_be_made() {
        let mut sut = MockBankManagerRepository::new();

        let id = Id::new(Ulid::new());
        let data = RenewMoney { money: 200 };

        sut.expect_update_money()
            .with(eq(id.clone()), eq(data.clone()))
            .times(1)
            .returning(|_, _| Ok(()));

        let result = sut.update_money(&id, data).await;

        assert!(result.is_ok());
    }
}
