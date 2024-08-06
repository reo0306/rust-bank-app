use crate::domain::{
    model::bank::BankAccount,
    repository::bank::BankManagerRepository,
};

use super::repository::DatabaseRepositoryImpl;
use crate::adapter::persistence::mysql::Db;

pub struct RepositoriesModule {
    bank_manager_repository: DatabaseRepositoryImpl<BankAccount>,
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let bank_manager_repository = DatabaseRepositoryImpl::new(db.clone());

        Self {
            bank_manager_repository,
        }
    }
}

pub trait RepositoriesModuleExt {
   type BankRepo: BankManagerRepository;

   fn bank_manager_repository(&self) -> &Self::BankRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type BankRepo = DatabaseRepositoryImpl<BankAccount>;

    fn bank_manager_repository(&self) -> &Self::BankRepo {
        &self.bank_manager_repository
    }
}