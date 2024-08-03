use anyhow::Result;
use std::sync::Arc;

use bank_app::driver::{
    modules::Modules,
    routes::startup,
};

#[tokio::main]
async fn main() -> Result<()> {
    let modules = Modules::new().await;

    startup::run(Arc::new(modules)).await.expect("run child care error");

    Ok(())
}