use std::sync::Arc;

use crate::adapter::{
    module::{RepositoriesModule, RepositoriesModuleExt},
    persistence::{
        dynamodb::{init_client, DynamoDB},
        mysql::Db,
    },
    repository::deposit_history::DepositHistoryRepository,
};
use crate::app::usecase::{bank::BankManagerUseCase, deposit_history::DepositHistoryUseCase, graphql::BankQueryUseCase};

pub struct Modules {
    pub bank_manager_use_case: BankManagerUseCase<RepositoriesModule>,
    pub deposit_history_use_case: DepositHistoryUseCase,
    pub bank_query_use_case: BankQueryUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn bank_manager_use_case(&self) -> &BankManagerUseCase<Self::RepositoriesModule>;
    fn deposit_history_use_case(&self) -> &DepositHistoryUseCase;
    fn bank_query_use_case(&self) -> &BankQueryUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn bank_manager_use_case(&self) -> &BankManagerUseCase<Self::RepositoriesModule> {
        &self.bank_manager_use_case
    }

    fn deposit_history_use_case(&self) -> &DepositHistoryUseCase {
        &self.deposit_history_use_case
    }

    fn bank_query_use_case(&self) -> &BankQueryUseCase<Self::RepositoriesModule> {
        &self.bank_query_use_case
    }
}

impl Modules {
    pub async fn new() -> Self {
        let db = Db::new().await;
        let client = init_client().await;
        let dynamodb = DynamoDB::new(client);

        let repositories_module = Arc::new(RepositoriesModule::new(db.clone()));

        let bank_manager_use_case = BankManagerUseCase::new(repositories_module.clone());
        let deposit_history_use_case =
            DepositHistoryUseCase::new(DepositHistoryRepository::new(dynamodb));
        let bank_query_use_case = BankQueryUseCase::new(repositories_module.clone());

        Self {
            bank_manager_use_case,
            deposit_history_use_case,
            bank_query_use_case,
        }
    }
}
