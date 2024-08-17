use anyhow::Result;

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
