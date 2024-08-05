use crate::domain::{
    model::bank::Bank,
    repository::bank::BankManagerRepository,
};

use super::repository::DatabaseRepositoryImpl;
use crate::adapter::database::mysql::Db;

pub struct RepositoriesModule {
    bank_repository: DatabaseRepositoryImpl<Bank>,
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let bank_repository = DatabaseRepositoryImpl::new(db.clone());

        Self {
            bank_repository,
        }
    }
}

pub trait RepositoriesModuleExt {
   type BankRepo: BabyRepository;

   fn bank_repository(&self) -> &Self::BankRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type BankRepo = DatabaseRepositoryImpl<Bank>;

    fn bank_repository(&self) -> &Self::BankRepo {
        &self.bank_repository
    }
}