use derive_new::new;

use crate::domain::model::Id;

#[derive(new, PartialEq, Debug, Clone)]
pub struct BankAccount {
    pub bank_id: String,
    pub branch_office_id: String,
    pub name: String,
    pub money: i32,
}

#[derive(new, PartialEq, Debug, Clone)]
pub struct DepositHistories {
    pub bank_account_id: String,
    pub action: String,
    pub money: i32,
}

#[derive(new, PartialEq, Debug, Clone)]
pub struct NewBankAccount {
    pub id: Id<BankAccount>,
    pub bank_id: String,
    pub branch_office_id: String,
    pub name: String,
    pub money: i32,
}

#[derive(new, PartialEq, Debug, Clone)]
pub struct NewDepositHistory {
    pub id: Id<DepositHistories>,
    pub bank_account_id: String,
    pub action: String,
    pub money: i32,
}

#[derive(new, PartialEq, Debug, Clone)]
pub struct RenewMoney {
    pub money: i32,
}
