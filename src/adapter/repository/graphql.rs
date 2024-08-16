use async_graphql::Object;

use crate::adapter::model::graphql::Ping;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
  async fn ping(&self) -> Ping {
    Ping { 
      status: "ok".to_string(), 
      code: 200 
    }
  }
}
