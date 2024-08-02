use dotenv::dotenv;
use sqlx::{MySql, MySqlPool, Pool};
use std::sync::Arc;

#[derive(Clone)]
pub struct Db(pub(crate) Arc<Pool<MySql>>);

impl Db {
    pub async fn new() -> Self {
        dotenv().ok();

        let user: String = dotenv::var("DB_USER").unwrap();
        let pass: String = dotenv::var("DB_PASS").unwrap();
        let host: String = dotenv::var("DB_HOST").unwrap();
        let port: String = dotenv::var("DB_PORT").unwrap();
        let database: String = dotenv::var("DATABASE").unwrap();

        let db_url: String = format!("mysql://{user}:{pass}@{host}:{port}/{database}");

        let pool = MySqlPool::connect(&db_url).await;

        pool.map_or_else(
            |_| panic!("not connect db"),
            |pool| Self(Arc::new(pool))
        )
    }
}