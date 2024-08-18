use crate::domain::{
    model::{
        bank::BankAccount,
        graphql::BankQueryAccount,
    },
    repository::{
        bank::BankManagerRepository,
        graphql::BankQueryRepository,
    },
};

use super::repository::DatabaseRepositoryImpl;
use crate::adapter::persistence::mysql::Db;

pub struct RepositoriesModule {
    bank_manager_repository: DatabaseRepositoryImpl<BankAccount>,
    bank_query_repository: DatabaseRepositoryImpl<BankQueryAccount>,
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let bank_manager_repository = DatabaseRepositoryImpl::new(db.clone());
        let bank_query_repository = DatabaseRepositoryImpl::new(db.clone());

        Self {
            bank_manager_repository,
            bank_query_repository,
        }
    }
}

pub trait RepositoriesModuleExt {
    type BankRepo: BankManagerRepository;
    type BankQueryRepo: BankQueryRepository;

    fn bank_manager_repository(&self) -> &Self::BankRepo;
    fn bank_query_repository(&self) -> &Self::BankQueryRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type BankRepo = DatabaseRepositoryImpl<BankAccount>;
    type BankQueryRepo = DatabaseRepositoryImpl<BankQueryAccount>;

    fn bank_manager_repository(&self) -> &Self::BankRepo {
        &self.bank_manager_repository
    }

    fn bank_query_repository(&self) -> &Self::BankQueryRepo {
        &self.bank_query_repository
    }
}
