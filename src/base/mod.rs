use hyper::body;
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;

use crate::error::{ApiError, Error, Result};

pub mod models;

pub const API_BASE: &str = "https://database.deta.sh/v1";

pub struct Base {
    project_id: String,
    project_key: String,
}

impl Base {
    pub(in crate) fn new(project_key: String) -> Self {
        Self {
            project_id: String::from(project_key.clone().split("_").next().unwrap()),
            project_key,
        }
    }

    pub async fn put_items<T: DeserializeOwned + Serialize>(
        &self,
        base_name: &str,
        items: models::Items<T>,
    ) -> Result<models::PutItemsResponse<T>> {
        let request = Request::put(format!(
            "{API_BASE}/{}/{}/items",
            self.project_id, base_name
        ))
        .header("Content-Type", "application/json")
        .header("X-Api-Key", &self.project_key)
        .body(Body::from(
            serde_json::to_string(&json!({
                "items": items,
            }))
            .unwrap(),
        ))?;

        let client = Client::builder().build::<_, Body>(HttpsConnector::new());
        let mut response = client.request(request).await?;
        let bytes = body::to_bytes(response.body_mut()).await?;

        if response.status().is_success() {
            Ok(serde_json::from_slice(bytes.as_ref())?)
        }
        else {
            Err(Error::from(serde_json::from_slice::<ApiError>(bytes.as_ref())?))
        }
    }
}
