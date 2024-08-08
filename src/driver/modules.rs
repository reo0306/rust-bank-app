use std::sync::Arc;

use crate::adapter::{
    module::{RepositoriesModule, RepositoriesModuleExt},
    persistence::mysql::Db,
};
use crate::app::usecase::bank::BankManagerUseCase;

pub struct Modules {
    pub bank_manager_use_case: BankManagerUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn bank_manager_use_case(&self) -> &BankManagerUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn bank_manager_use_case(&self) -> &BankManagerUseCase<Self::RepositoriesModule> {
        &self.bank_manager_use_case
    }
}

impl Modules {
    pub async fn new() -> Self {
        let db = Db::new().await;

        let repositories_module = Arc::new(RepositoriesModule::new(db.clone()));

        let bank_manager_use_case = BankManagerUseCase::new(repositories_module.clone());

        Self {
            bank_manager_use_case,
        }
    }
}
