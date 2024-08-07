use derive_new::new;
use std::sync::Arc;
use anyhow::Result;

use crate::domain::{
    model::bank::{BankAccount, DepositHistories},
    repository::bank::BankManagerRepository
};
use crate::adapter::module::RepositoriesModuleExt;
use crate::app::model::bank::{CreateBankAccount, CreateDepositHistory, UpdateMoney};

#[derive(new)]
pub struct BankManagerUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl <R: RepositoriesModuleExt> BankManagerUseCase<R> {
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
            .create_new_account(data.try_into()?).await
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

    pub async fn manage_money(&self, id: String, data: UpdateMoney) -> Result<()> {
        self.repositories
            .bank_manager_repository()
            .update_money(&id.try_into()?, data.try_into()?)
            .await
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