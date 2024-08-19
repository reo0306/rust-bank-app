use async_graphql::SimpleObject;
use derive_new::new;

use crate::domain::model::Id;

#[derive(SimpleObject)]
pub struct Ping {
    pub status: String,
    pub code: i32,
}

#[derive(SimpleObject)]
pub struct BankQueryAccount {
    pub bank_id: String,
    pub branch_office_id: String,
    pub name: String,
    pub money: i32,
}

#[derive(new)]
pub struct NewBankMutationAccount {
    pub id: Id<BankQueryAccount>,
    pub bank_id: String,
    pub branch_office_id: String,
    pub name: String,
    pub money: i32,
}