use derive_new::new;

use crate::domain::model::{
    graphql::NewBankMutationAccount,
    Id,
};

#[derive(new)]
pub struct CreateBankMutationAccount {
    pub bank_id: String,
    pub branch_office_id: String,
    pub name: String,
    pub money: i32,
}

impl TryFrom<CreateBankMutationAccount> for NewBankMutationAccount {
    type Error = anyhow::Error;

    fn try_from(cba: CreateBankMutationAccount) -> anyhow::Result<Self, Self::Error> {
        let new_bank_account_id = Id::gen();

        Ok(NewBankMutationAccount::new(
            new_bank_account_id,
            cba.bank_id,
            cba.branch_office_id,
            cba.name,
            cba.money,
        ))
    }
}
