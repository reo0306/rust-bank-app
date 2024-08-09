use serde::{Deserialize, Serialize};

use crate::app::model::bank::{CreateBankAccount, CreateDepositHistory, UpdateMoney};
use crate::domain::model::bank::{BankAccount, DepositHistories};

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
    pub money: i32,
}

impl From<JsonCreateAccount> for CreateBankAccount {
    fn from(jca: JsonCreateAccount) -> Self {
        CreateBankAccount {
            bank_id: jca.bank_id,
            branch_office_id: jca.branch_office_id,
            name: jca.name,
            money: jca.money,
        }
    }
}

impl From<JsonCreateHistory> for CreateDepositHistory {
    fn from(jch: JsonCreateHistory) -> Self {
        CreateDepositHistory {
            bank_account_id: jch.bank_account_id,
            action: jch.action,
            money: jch.money,
        }
    }
}

impl From<JsonUpdateMoney> for UpdateMoney {
    fn from(jum: JsonUpdateMoney) -> Self {
        UpdateMoney { money: jum.money }
    }
}

impl From<BankAccount> for JsonAccountView {
    fn from(ba: BankAccount) -> Self {
        JsonAccountView {
            bank_id: ba.bank_id.to_string(),
            branch_office_id: ba.branch_office_id.to_string(),
            name: ba.name,
            money: ba.money,
        }
    }
}

impl From<DepositHistories> for JsonHistoriesView {
    fn from(dh: DepositHistories) -> Self {
        JsonHistoriesView {
            action: dh.action,
            money: dh.money,
        }
    }
}
