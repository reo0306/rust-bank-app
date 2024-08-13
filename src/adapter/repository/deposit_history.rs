use anyhow::Result;
use std::sync::Arc;
use aws_sdk_dynamodb::types::AttributeValue;

use crate::adapter::{
    persistence::dynamodb::DynamoDB,
    model::bank::{DepositHistoriesTable, NewDepositHistoryRecord},
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

    /*pub async fn find_histories(&self, id: &Id<DepositHistories>) -> Result<Option<DepositHistories>> {
        let deposit_histories_table = self.dynamodb
            .client
            .get_item()
            .table_name("DepositHistories")
            .key("id".to_string(), AttributeValue::S(id.value.to_string()))
            .send()
            .await?;
println!("{:?}", deposit_histories_table);
        //if let Some(item) = deposit_histories_table.item {
        //}
        //deposit_histories_table.map_or(Ok(None), |item| Ok(Some(item.try_into()?)))
    }*/

    pub async fn create_new_history(&self, params: NewDepositHistory) -> Result<()> {
        let new_deposit_history_record: NewDepositHistoryRecord = params.try_into()?;

        self.dynamodb
            .client
            .put_item()
            .table_name("DepositHistories")
            .item("id", AttributeValue::S(new_deposit_history_record.id))
            .item("bank_account_id", AttributeValue::S(new_deposit_history_record.bank_account_id))
            .item("action", AttributeValue::S(new_deposit_history_record.action))
            .item("money", AttributeValue::N(new_deposit_history_record.money.to_string()))
            //.return_values(ReturnValue::ALLOld)
            .send()
            .await?;

        Ok(())
    }
}
