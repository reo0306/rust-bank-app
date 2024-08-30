use serde::{Deserialize, Serialize};

use crate::app::model::bank::{CreateBankAccount, CreateDepositHistory, UpdateMoney, LoginBankAccount};
use crate::domain::model::bank::{BankAccount, DepositHistories, DepositDownloadHistories};

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

#[derive(Debug, Serialize)]
pub struct JsonHistoriesDownload {
    pub id: String,
    pub bank_account_id: String,
    pub action: String,
    pub money: i32,
}

#[derive(Debug, Deserialize)]
pub struct JsonCreateAccount {
    pub bank_id: String,
    pub branch_office_id: String,
    pub name: String,
    pub password: String,
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

#[derive(Debug, Serialize)]
pub struct JsonLogin {
    pub id: String,
    pub password: String,
}

impl From<JsonCreateAccount> for CreateBankAccount {
    fn from(jca: JsonCreateAccount) -> Self {
        CreateBankAccount {
            bank_id: jca.bank_id,
            branch_office_id: jca.branch_office_id,
            name: jca.name,
            password: jca.password,
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

impl From<DepositDownloadHistories> for JsonHistoriesDownload {
    fn from(ddh: DepositDownloadHistories) -> Self {
        JsonHistoriesDownload {
            id: ddh.id,
            bank_account_id: ddh.bank_account_id,
            action: ddh.action,
            money: ddh.money,
        }
    }
}

impl From<JsonLogin> for LoginAccount {
    fn from(jlv: JsonLogin) -> Self {
        LoginAccount{
            id: jlv.id,
            password: jlv.password,
        }
    }
}