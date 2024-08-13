use serde::{Serialize, Deserialize};
use crate::domain::model::bank::{DepositHistories, NewDepositHistory};

#[derive(Debug, Serialize, Deserialize)]
pub struct DepositHistoriesDynamoTable {
    pub bank_account_id: String,
    pub action: String,
    pub money: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewDepositHistoryDynamoRecord {
    pub id: String,
    pub bank_account_id: String,
    pub action: String,
    pub money: i32,
}

impl TryFrom<DepositHistoriesDynamoTable> for DepositHistories {
    type Error = anyhow::Error;

    fn try_from(dhdt: DepositHistoriesDynamoTable) -> Result<Self, Self::Error> {
        Ok(DepositHistories {
            bank_account_id: dhdt.bank_account_id,
            action: dhdt.action,
            money: dhdt.money,
        })
    }
}

impl TryFrom<NewDepositHistory> for NewDepositHistoryDynamoRecord {
    type Error = anyhow::Error;

    fn try_from(ndh: NewDepositHistory) -> Result<Self, Self::Error> {
        Ok(NewDepositHistoryDynamoRecord {
            id: ndh.id.value.to_string(),
            bank_account_id: ndh.bank_account_id,
            action: ndh.action,
            money: ndh.money,
        })
    }
}