use anyhow::Result;
use std::sync::Arc;
use aws_sdk_dynamodb::types::AttributeValue;
use serde_dynamo::aws_sdk_dynamodb_1::{from_item, to_item};

use crate::adapter::{
    persistence::dynamodb::DynamoDB,
    model::deposit_history::{DepositHistoriesDynamoTable, NewDepositHistoryDynamoRecord},
};
use crate::domain::{
    model::{
        bank::{DepositHistories, NewDepositHistory},
        Id,
    },
};

pub struct DepositHistoryRepository {
    dynamodb: Arc<DynamoDB>,
}

impl DepositHistoryRepository {
    pub fn new(dynamodb: DynamoDB) -> Self {
        Self {
            dynamodb: Arc::new(dynamodb),
        }
    }

    pub async fn find_history(&self, id: &Id<DepositHistories>) -> Result<Option<DepositHistories>> {
        let resp = self.dynamodb
            .client
            .get_item()
            .table_name("DepositHistories")
            .key("id".to_string(), AttributeValue::S(id.value.to_string()))
            .send()
            .await?;

        let depoisit_historiy_dynamo_table: DepositHistoriesDynamoTable = from_item(resp.item.clone().unwrap()).unwrap();

        Ok(Some(depoisit_historiy_dynamo_table.try_into()?))
    }

    pub async fn create_new_history(&self, params: NewDepositHistory) -> Result<()> {
        let new_deposit_history_record: NewDepositHistoryDynamoRecord = params.try_into()?;

        self.dynamodb
            .client
            .put_item()
            .table_name("DepositHistories")
            .set_item(Some(to_item(new_deposit_history_record).unwrap()))
            .send()
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod deposit_history_dynamodb_test {
    use ulid::Ulid;

    use super::*;
    use crate::adapter::persistence::dynamodb::{init_client, DynamoDB};
    use crate::domain::model::{
        Id,
        bank::NewDepositHistory
    };

    #[tokio::test]
    async fn it_create_new_history() {
        let client = init_client().await;
        let dynamodb = DynamoDB::new(client);
        let repository = DepositHistoryRepository::new(dynamodb);

        let result = repository.create_new_history(NewDepositHistory {
            id: Id::new(Ulid::new()),
            bank_account_id: "aaaa".to_string(),
            action: "test".to_string(),
            money: 9999,
        }
        ).await;

        assert!(result.is_ok());
    }
}