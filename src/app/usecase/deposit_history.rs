use anyhow::Result;
use std::sync::Arc;

use crate::adapter::repository::deposit_history::DepositHistoryRepository;
use crate::app::model::bank::CreateDepositHistory;
use crate::domain::model::bank::DepositHistories;

pub struct DepositHistoryUseCase {
    repository: Arc<DepositHistoryRepository>,
}

impl DepositHistoryUseCase {
    pub fn new(repository: DepositHistoryRepository) -> Self {
        Self {
            repository: Arc::new(repository),
        }
    }

    /*pub async fn view_histories(&self, id: String) -> Result<Option<DepositHistories>> {
        self.repository
            .find_histories(&id.try_into()?)
            .await
            .map(|histories| histories.map(|h| h.into()))
    }*/

    pub async fn add_history(&self, data: CreateDepositHistory) -> Result<()> {
        self.repository
            .create_new_history(data.try_into()?)
            .await
    }
}