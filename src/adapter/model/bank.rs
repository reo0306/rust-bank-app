use sqlx::FromRow;

use crate::domain::model::bank::{
    BankAccount, DepositHistories, NewBankAccount, NewDepositHistory, RenewMoney,
};

#[derive(FromRow)]
pub struct BankAccountTable {
    pub bank_id: String,
    pub branch_office_id: String,
    pub name: String,
    pub money: i32,
}

#[derive(FromRow)]
pub struct DepositHistoriesTable {
    pub bank_account_id: String,
    pub action: String,
    pub money: i32,
}

pub struct NewBankAccountRecord {
    pub id: String,
    pub bank_id: String,
    pub branch_office_id: String,
    pub name: String,
    pub money: i32,
}

pub struct NewDepositHistoryRecord {
    pub id: String,
    pub bank_account_id: String,
    pub action: String,
    pub money: i32,
}

pub struct RenewMoneyRecord {
    pub money: i32,
}

impl TryFrom<BankAccountTable> for BankAccount {
    type Error = anyhow::Error;

    fn try_from(bat: BankAccountTable) -> Result<Self, Self::Error> {
        Ok(BankAccount {
            bank_id: bat.bank_id,
            branch_office_id: bat.branch_office_id,
            name: bat.name,
            money: bat.money,
        })
    }
}

impl TryFrom<DepositHistoriesTable> for DepositHistories {
    type Error = anyhow::Error;

    fn try_from(dht: DepositHistoriesTable) -> Result<Self, Self::Error> {
        Ok(DepositHistories {
            bank_account_id: dht.bank_account_id,
            action: dht.action,
            money: dht.money,
        })
    }
}

impl TryFrom<NewBankAccount> for NewBankAccountRecord {
    type Error = anyhow::Error;

    fn try_from(nba: NewBankAccount) -> Result<Self, Self::Error> {
        Ok(NewBankAccountRecord {
            id: nba.id.value.to_string(),
            bank_id: nba.bank_id,
            branch_office_id: nba.branch_office_id,
            name: nba.name,
            money: nba.money,
        })
    }
}

impl TryFrom<NewDepositHistory> for NewDepositHistoryRecord {
    type Error = anyhow::Error;

    fn try_from(ndh: NewDepositHistory) -> Result<Self, Self::Error> {
        Ok(NewDepositHistoryRecord {
            id: ndh.id.value.to_string(),
            bank_account_id: ndh.bank_account_id,
            action: ndh.action,
            money: ndh.money,
        })
    }
}

impl TryFrom<RenewMoney> for RenewMoneyRecord {
    type Error = anyhow::Error;

    fn try_from(rm: RenewMoney) -> Result<Self, Self::Error> {
        Ok(RenewMoneyRecord { money: rm.money })
    }
}
