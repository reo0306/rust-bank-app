use sqlx::FromRow;

use crate::domain::model::graphql::{BankQueryAccount, NewBankMutationAccount};

#[derive(FromRow)]
pub struct BankQueryAccountTable {
    pub bank_id: String,
    pub branch_office_id: String,
    pub name: String,
    pub money: i32,
}

pub struct NewBankMutationAccountRecord {
    pub id: String,
    pub bank_id: String,
    pub branch_office_id: String,
    pub name: String,
    pub money: i32,
}

impl TryFrom<BankQueryAccountTable> for BankQueryAccount {
    type Error = anyhow::Error;

    fn try_from(bat: BankQueryAccountTable) -> Result<Self, Self::Error> {
        Ok(BankQueryAccount {
            bank_id: bat.bank_id,
            branch_office_id: bat.branch_office_id,
            name: bat.name,
            money: bat.money,
        })
    }
}

impl TryFrom<NewBankMutationAccount> for NewBankMutationAccountRecord {
    type Error = anyhow::Error;

    fn try_from(nba: NewBankMutationAccount) -> Result<Self, Self::Error> {
        Ok(NewBankMutationAccountRecord {
            id: nba.id.value.to_string(),
            bank_id: nba.bank_id,
            branch_office_id: nba.branch_office_id,
            name: nba.name,
            money: nba.money,
        })
    }
}