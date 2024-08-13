use std::sync::Arc;
use anyhow::Result;
use anyhow::anyhow;

use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::config::Builder;
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;
use serde_dynamo::aws_sdk_dynamodb_1::from_item;

use crate::domain::model::bank::DepositHistories;
use crate::adapter::model::deposit_history::DepositHistoriesDynamoTable;

/*
use aws_config::load_from_env;
use aws_sdk_dynamodb::config::{
    endpoint::{ResolveEndpoint, EndpointFuture, Params, Endpoint},
    Builder,
};
use aws_sdk_dynamodb::Client;
use http::Uri;
*/

/*
#[derive(Debug)]
pub struct LocalResolver { local: String }

impl ResolveEndpoint for LocalResolver {
    fn resolve_endpoint(&self, _params: &Params) -> EndpointFuture<'_> {
        let local = &self.local;
        EndpointFuture::ready(Ok(Endpoint::builder().url(format!("{local}")).build()))
    }
}*/

pub struct DynamoDB {
    pub(crate) client: Arc<Client>,
}

pub async fn init_client() -> Client {
    //let config = load_from_env().await;
    /*
    let config = Builder::from(&config)
        .endpoint_resolver(Endpoint::immutable(Uri::from_static(
            "http://localhost:800",
        )))
        .build();

    let dynamodb = Client::from_conf(config);
    */

    /*
    let resolver = LocalResolver { local: "http://localhost:8000".to_string() };
    let config = aws_sdk_dynamodb::Config::builder().endpoint_resolver(resolver).build();
    let dynamodb = Client::from_conf(config);
    */

    let config = aws_config::defaults(BehaviorVersion::latest())
        .test_credentials()
        .load()
        .await;

    let dynamodb_local_config = Builder::from(&config)
        .endpoint_url(
            "http://172.21.0.7:8000",
        )
        .build();

    let dynamodb = Client::from_conf(dynamodb_local_config);

    dynamodb
}

impl DynamoDB {
    pub fn new(client: Client) -> DynamoDB {
        DynamoDB {
            client: Arc::new(client),
        }
    }

    pub async fn list_tables(&self) -> Result<Option<Vec<String>>> {
        let res = self.client.list_tables().send().await?;

        Ok(res.table_names)
    }

    pub async fn get_items(&self) -> Result<Option<HashMap<String, AttributeValue>>> {
        let res = self.client.get_item()
        .table_name("DepositHistories")
        .key("id".to_string(), AttributeValue::S("1".to_string()))
        .send()
        .await?;

        let dh: DepositHistoriesDynamoTable = from_item(res.item.clone().unwrap()).unwrap();
        println!("{:?}", dh);
        let dhdt: DepositHistories = dh.try_into()?;
        println!("{:?}", dhdt);
        /*for items in res.item.unwrap() {
            let dh: DepositHistoriesDynamoTable = from_item(items).unwrap();
            println!("{:?}", dh);
        }*/
//println!("{:?}", res.item.clone().expect("err").get("id").ok_or_else(|| anyhow!("error"))?.as_s());
        Ok(res.item)
    }
}

#[cfg(test)]
mod dynamodb_test {
    use super::*;

    #[tokio::test]
    async fn test_list_tabel() {
        let client = init_client().await;
        let dynamodb = DynamoDB::new(client);
        let r = dynamodb.list_tables().await;

        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_get_items() {
        let client = init_client().await;
        let dynamodb = DynamoDB::new(client);
        let r = dynamodb.get_items().await;

        assert!(r.is_ok());
    }
}