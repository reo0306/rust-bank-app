use serde::Deserialize;

use crate::app::model::{
    CreateAccount,
    CreateHistory,
    UpdateMoney,
};

#[derive(Debug, Serialize)]
pub struct JsonAccountView {
    pub bank_id: String,
    pub branch_office_id: String,
    pub name: String,
    pub money: i32,
}

#[derive(Debug, Serialize)]
pub struct JsonHistoriesView {
    pub action: String,
    pub money: i32,
}

#[derive(Debug, Deserialize)]
pub struct JsonCreateAccount {
    pub bank_id: String,
    pub branch_office_id: String,
    pub name: String,
    pub money: i32,
}

#[derive(Debug, Deserialize)]
pub struct JsonCreateHistory {
    pub bank_account_id: String,
    pub action: String,
    pub money: i32,
}

#[derive(Debug, Deserialize)]
pub struct JsonUpdateMoney {
    pub id: String,
    pub money: i32,
}

impl From<JsonCreateAccount> for CreateAccount {
    fn from(jca: JsonCreateAccount) -> Self {
        CreateAccount {
            bank_id: jca.bank_id,
            branch_office_id: jca.branch_office_id,
            name: jca.name,
            money: jca.money,
        }
    }
}

impl From<JsonCreateHistory> for CreateHistory {
    fn from(jch: JsonCreateHistory) -> Self {
        CreateHistory {
            bank_account_id: jch.bank_account_id,
            action: jca.action,
            money: jca.money,
        }
    }
}

impl From<JsonUpdateMoney> for UpdateMoney {
    fn from(jum: JsonUpdateMoney) -> Self {
        UpdateBaby {
            id: jum.id,
            monet: jum.money,
        } 
    }
}

impl From<BankAccount> for JsonAccountView {
    fn from(ba: BankAccount) -> Self {
        JsonAccountView {
            bank_id: ba.id.value.to_string(),
            branch_office_id: ba.id.value.to_string(),
            name: ba.name,
            money: ba.money,
        }
    }
}

impl From<DepositHistories> for JsonHistoryView {
    fn from(dh: DepositHistories) -> Self {
        JsonHistoryView {
            action: dh.action,
            money: dh.money,
        }
    }
}