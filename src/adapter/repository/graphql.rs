use anyhow::Result;
use async_trait::async_trait;

use super::DatabaseRepositoryImpl;
use crate::adapter::model::graphql::{BankQueryAccountTable, NewBankMutationAccountRecord};
use crate::domain::{
    model::{
        Id,
        graphql::{Ping, BankQueryAccount, NewBankMutationAccount},
    },
    repository::graphql::BankQueryRepository,
};

#[async_trait]
impl BankQueryRepository for DatabaseRepositoryImpl<BankQueryAccount> {
    async fn ping(&self) -> Result<Ping> {
        Ok(Ping {
            status: "ok".to_string(),
            code: 200,
        })
    }

    async fn find_account(&self, id: &Id<BankQueryAccount>) -> Result<Option<BankQueryAccount>> {
        let pool = self.pool.0.clone();

        let bank_account_table = sqlx::query_as::<_, BankQueryAccountTable>(
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

    async fn create_new_account(&self, params: NewBankMutationAccount) -> Result<Option<BankQueryAccount>> {
        let pool = self.pool.0.clone();

        let new_bank_account_record: NewBankMutationAccountRecord = params.try_into()?;

        sqlx::query(
            r#"
            INSERT INTO bank_accounts (id, bank_id, branch_office_id, name, money) VALUES(?, ?, ?, ?, ?);
            "#,
        )
        .bind(&new_bank_account_record.id)
        .bind(&new_bank_account_record.bank_id)
        .bind(&new_bank_account_record.branch_office_id)
        .bind(&new_bank_account_record.name)
        .bind(new_bank_account_record.money)
        .execute(&*pool)
        .await?;

        let res = sqlx::query_as::<_, BankQueryAccountTable>(
            r#"
            SELECT bank_id, branch_office_id, name, money from bank_accounts WHERE id = ?;
            "#,
        )
        .bind(&new_bank_account_record.id)
        .fetch_one(&*pool)
        .await
        .ok();

        res.map_or(Ok(None), |data| Ok(Some(data.try_into()?)))
    }
}