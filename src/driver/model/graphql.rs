use async_graphql::InputObject;
use crate::app::model::graphql::CreateBankMutationAccount;

#[derive(InputObject)]
pub struct CreateAccount {
    pub bank_id: String,
    pub branch_office_id: String,
    pub name: String,
    pub money: i32,
}

impl From<CreateAccount> for CreateBankMutationAccount {
    fn from(ca: CreateAccount) -> Self {
        CreateBankMutationAccount{
            bank_id: ca.bank_id,
            branch_office_id: ca.branch_office_id,
            name: ca.name,
            money: ca.money,
        }
    }
}