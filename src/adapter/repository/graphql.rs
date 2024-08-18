use anyhow::Result;
use async_trait::async_trait;

use super::DatabaseRepositoryImpl;
use crate::adapter::model::graphql::BankQueryAccountTable;
use crate::domain::{
    model::{
        Id,
        graphql::{Ping, BankQueryAccount},
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
}

/*use anyhow::Result;

use crate::adapter::persistence::mysql::Db;
use crate::domain::model::graphql::Ping;

pub struct BankQueryRepository {
    db: Db,
}

impl BankQueryRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn ping(&self) -> Result<Ping> {
        Ok(Ping {
            status: "ok".to_string(),
            code: 200,
        })
    }
}
*/