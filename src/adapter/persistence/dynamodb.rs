use std::sync::Arc;
use anyhow::Result;

use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::config::Builder;

/*
use aws_sdk_dynamodb::config::endpoint::{Endpoint, EndpointFuture, Params, ResolveEndpoint};

#[derive(Debug)]
pub struct LocalResolver { local: String }

impl ResolveEndpoint for LocalResolver {
    fn resolve_endpoint(&self, _params: &Params) -> EndpointFuture<'_> {
        let local = &self.local;
        EndpointFuture::ready(Ok(Endpoint::builder().url(format!("{local}")).build()))
    }
}
*/

pub struct DynamoDB {
    pub(crate) client: Arc<Client>,
}

pub async fn init_client() -> Client {
    /*
    let resolver = LocalResolver { local: "http://172.21.0.7:8000".to_string() };
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
}

#[cfg(test)]
mod dynamodb_test {
    use aws_sdk_dynamodb::types::AttributeValue;
    use std::collections::HashMap;

    use super::*;

    #[tokio::test]
    async fn test_list_tabel() {
        let client = init_client().await;
        let dynamodb = DynamoDB::new(client);
        let r = dynamodb.list_tables().await;

        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_get_item() {
        let r = get_item("1".to_string()).await;

        assert!(r.is_ok());
    }

    async fn get_item(id: String) -> Result<Option<HashMap<String, AttributeValue>>> {
        let client = init_client().await;
        let dynamodb = DynamoDB::new(client);
        let res = dynamodb.client.get_item()
        .table_name("DepositHistories")
        .key("id".to_string(), AttributeValue::S(id))
        .send()
        .await?;

        Ok(res.item)
    }
}