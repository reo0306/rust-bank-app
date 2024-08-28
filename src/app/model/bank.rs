use derive_new::new;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        SaltString,
        PasswordHasher,
    },
    Algorithm,
    Version,
    Params,
    Argon2
};

use crate::domain::model::{
    bank::{NewBankAccount, NewDepositHistory, RenewMoney},
    Id,
};

#[derive(new)]
pub struct CreateBankAccount {
    pub bank_id: String,
    pub branch_office_id: String,
    pub name: String,
    pub password: String,
    pub money: i32,
}

#[derive(new)]
pub struct CreateDepositHistory {
    pub bank_account_id: String,
    pub action: String,
    pub money: i32,
}

#[derive(new)]
pub struct UpdateMoney {
    pub money: i32,
}

impl TryFrom<CreateBankAccount> for NewBankAccount {
    type Error = anyhow::Error;

    fn try_from(cba: CreateBankAccount) -> anyhow::Result<Self, Self::Error> {
        let new_bank_account_id = Id::gen();

        let salt = SaltString::generate(&mut OsRng);
        //let argo2 = Argon2::default();
        //let password_hash = argo2.hash_password(cba.password.as_bytes(), &salt).unwrap().to_string();

        //let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        )
        .hash_password(cba.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

        Ok(NewBankAccount::new(
            new_bank_account_id,
            cba.bank_id,
            cba.branch_office_id,
            cba.name,
            password_hash,
            cba.money,
        ))
    }
}

impl TryFrom<CreateDepositHistory> for NewDepositHistory {
    type Error = anyhow::Error;

    fn try_from(cdh: CreateDepositHistory) -> anyhow::Result<Self, Self::Error> {
        let new_deposit_history_id = Id::gen();

        Ok(NewDepositHistory::new(
            new_deposit_history_id,
            cdh.bank_account_id,
            cdh.action,
            cdh.money,
        ))
    }
}

impl TryFrom<UpdateMoney> for RenewMoney {
    type Error = anyhow::Error;

    fn try_from(um: UpdateMoney) -> anyhow::Result<Self, Self::Error> {
        Ok(RenewMoney::new(um.money))
    }
}
