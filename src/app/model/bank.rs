use derive_new::new;

use crate::domain::model::{
    Id,
    bank::{NewBankAccount, NewDepositHistory, RenewMoney}
};

#[derive(new)]
pub struct CreateBankAccount {
    pub bank_id: String,
    pub branch_office_id: String,
    pub name: String,
    pub money: i32,
}

#[derive(new)]
pub struct CreateDepositoHistory {
    pub bank_account_id: String,
    pub action: String,
    pub money: i32,
}

#[derive(new)]
pub struct UpdateMoney {
    pub id: String,
    pub money: i32,
}

impl TryFrom<CreateBankAccount> for NewBankAccount {
    type Error = anyhow::Error;

    fn try_from(cba: CreateBankAccount) -> anyhow::Result<Self, Self::Error> {
        let new_bank_account_id = Id::gen();

        Ok(
            NewBankAccount::new(
                new_bank_account_id,
                cba.bank_id,
                cba.branch_office_id,
                cba.name,
                cba.money,
            )
        )
    }
}

impl TryFrom<CreateDepositoHistory> for NewDepositHistory {
    type Error = anyhow::Error;

    fn try_from(cdh: CreateDepositoHistory) -> anyhow::Result<Self, Self::Error> {
        let new_deposit_history_id = Id::gen();

        Ok(
            NewDepositHistory::new(
                new_deposit_history_id,
                cdh.bank_account_id,
                cdh.action,
                cdh.money,
            )
        )
    }
}

impl TryFrom<UpdateMoney> for RenewMoney {
    type Error = anyhow::Error;

    fn try_from(um: UpdateMoney) -> anyhow::Result<Self, Self::Error> {
        Ok(
            RenewMoney::new(
                um.id,
                um.money,
            )
        )
    }
}