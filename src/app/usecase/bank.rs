use derive_new::new;
use std::sync::Arc;
use anyhow::Result;

use crate::domain::{
    model::bank::Bank,
    repository::bank::BankManagerRepository
};
use crate::adapter::module::RepositoriesModuleExt;
use crate::app::model::bank::{CreateBaby, UpdateBaby};

#[derive(new)]
pub struct BankManagerUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl <R: RepositoriesModuleExt> BankManagerUseCase<R> {
    pub async fn find_baby(&self, id: String) -> Result<Option<Baby>> {
        self.repositories
            .baby_repository()
            .find(&id.try_into()?)
            .await
            .map(|baby| baby.map(|b| b.into()))
    }

    pub async fn create_baby(&self, data: CreateBaby) -> Result<()> {
        self.repositories.baby_repository().register(data.try_into()?).await
    }

    pub async fn update_baby(&self, id: String, data: UpdateBaby) -> Result<()> {
        self.repositories
            .baby_repository()
            .update(&id.try_into()?, data.try_into()?)
            .await
    }

    pub async fn delete_baby(&self, id: String) -> Result<()> {
        self.repositories.baby_repository().delete(&id.try_into()?).await
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