use anyhow::Result;
use derive_new::new;
use std::sync::Arc;
use argon2::{
    Argon2,
    password_hash::PasswordHash, PasswordVerifier,
};

use crate::adapter::module::RepositoriesModuleExt;
use crate::app::model::bank::{CreateBankAccount, CreateDepositHistory, UpdateMoney, LoginAccount,};
use crate::domain::{
    model::bank::{BankAccount, DepositHistories, DepositDownloadHistories,},
    repository::bank::BankManagerRepository,
};

#[derive(new)]
pub struct BankManagerUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> BankManagerUseCase<R> {
    pub async fn view_account(&self, id: String) -> Result<Option<BankAccount>> {
        self.repositories
            .bank_manager_repository()
            .find_account(&id.try_into()?)
            .await
            .map(|account| account.map(|a| a.into()))
    }

    pub async fn add_account(&self, data: CreateBankAccount) -> Result<()> {
        self.repositories
            .bank_manager_repository()
            .create_new_account(data.try_into()?)
            .await
    }

    pub async fn view_histories(&self, id: String) -> Result<Option<DepositHistories>> {
        self.repositories
            .bank_manager_repository()
            .find_histories(&id.try_into()?)
            .await
            .map(|histories| histories.map(|h| h.into()))
    }

    pub async fn add_history(&self, data: CreateDepositHistory) -> Result<()> {
        self.repositories
            .bank_manager_repository()
            .create_new_history(data.try_into()?)
            .await
    }

    pub async fn download_histories(&self, id: String) -> Result<Option<Vec<DepositDownloadHistories>>> {
        self.repositories
            .bank_manager_repository()
            .find_download_histories(&id.try_into()?)
            .await
            .map(|download_histories| download_histories.map(|dh| dh.into()))
    }


    pub async fn manage_money(&self, id: String, data: UpdateMoney) -> Result<()> {
        self.repositories
            .bank_manager_repository()
            .update_money(&id.try_into()?, data.try_into()?)
            .await
    }

    //pub async fn signup_account(&self, data: LoginAccount) -> Result<bool> {
    pub async fn signup_account(&self, data: LoginAccount) -> Result<()> {
    //pub async fn signup_account(&self, data: LoginAccount) -> Result<Option<SignupBankAccount>> {
        /*self.repositories
            .bank_manager_repository()
            .find_login_account(&data.id.try_into()?)
            .await
            .map(|account| account.map(|a| a.into()))
            */
        /*let account = self.repositories
            .bank_manager_repository()
            .find_login_account(&data.id.try_into()?)
            .await?;

        match account {
            Some(account) => {
                let pared_hash = PasswordHash::new(&account.password).unwrap();
                Argon2::default().verify_password(data.password.as_bytes(), &pared_hash)
                .map(|_| Ok(()))
                .map_err(|_| anyhow::Error::msg("Password verification failed"))
            }
            None => Err("error"),
        }*/
        self.repositories
            .bank_manager_repository()
            .find_login_account(&data.id.try_into()?)
            .await?
            .map(|account| {
                let pared_hash = PasswordHash::new(&account.password).unwrap();
                let result = Argon2::default().verify_password(data.password.as_bytes(), &pared_hash).is_ok();

                if result == true {
                    Ok(())
                } else {
                    Err(anyhow::Error::msg("Password verification failed"))
                }
                //.map(|_| ())
                //.map_err(|_| anyhow::Error::msg("Password verification failed"))
            }).expect("error")


                //.map_err(|_| false)
            /*.map_or(Ok(false), |account| {
                let pared_hash = PasswordHash::new(&account.password).unwrap();
                Argon2::default().verify_password(data.password.as_bytes(), &pared_hash)
                .map(|_| true)
                .map_err(|_| anyhow::Error::msg("Password verification failed"))
                //.map_err(|_| false)
            })*/
        /*
            .map_or(Ok(false), |account| {
            //.map_or(Err(_), |account| {
                let pared_hash = PasswordHash::new(&account.password).unwrap();
                //let pared_hash = PasswordHash::new(&account.password).map_err(|_| anyhow::Error::msg("Password parse error"))?;
                Argon2::default().verify_password(data.password.as_bytes(), &pared_hash)
                .map(|_| true)
                //.map(|_| ())
                //.map_err(|_| anyhow::Error::msg("Password verification failed"))
                .map_err(|_| Err(false))
            })
       */
            /*
            .map(|login_account| 
                login_account.map(|account| {
                    let pared_hash = PasswordHash::new(&account.password).unwrap();
                    Argon2::default().verify_password(data.password.as_bytes(), &pared_hash).is_ok()
                    /*match la {
                        Ok(account) => {
                            let pared_hash = PasswordHash::new(&account.password).unwrap();
                            Ok(Argon2::default().verify_password(data.password.as_bytes(), &pared_hash).is_ok())
                        }
                        Err(_) => Err(false)
                    }*/
                })
                .ok_or_else(|| false)
            );*/
    }

}

/*#[cfg(test)]
mod baby_repository_test {
    use mockall::predicate::*;
    use std::sync::Arc;

    use super::*;
    use crate::domain::{
        model::baby::Baby,
        repository::baby::MockBabyRepository,
    };
    use crate::adapter::{
        module::RepositoriesModule,
        database::mysql::Db,
    };

    #[tokio::test]
    async fn it_find_baby() {
        let id = baby.id;

        let mut mock_repo = MockBabyRepository::new();
        mock_repo
            .expect_find()
            .with(predicate::eq(id))
            .times(1)
            .return_const(Ok(baby));

        let modules = RepositoriesModule::new(Db::new().await);
        let baby_use_case = BabyUseCase::new(Arc::new(modules));

        let baby2 = baby_use_case.find_baby(id.value.to_string()).await;

        assert_eq!(baby, baby2.unwrap().expect("error"));
    }
}*/
