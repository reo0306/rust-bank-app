use derive_new::new;
use std::marker::PhantomData;

use super::persistence::mysql::Db;

pub mod bank;

#[derive(new)]
pub struct DatabaseRepositoryImpl<T> {
    pool: Db,
    _maker: PhantomData<T>,
}
