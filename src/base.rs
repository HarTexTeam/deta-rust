use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::json;

use crate::error::Result;

pub const API_BASE: &str = "https://database.deta.sh/v1";

pub struct Base {
    project_id: String,
    project_key: String,
}

impl Base {
    pub(in crate) fn new(project_id: String, project_key: String) -> Self {
        Self { project_id, project_key }
    }

    // TODO: change return type for a more verbose response from Deta HTTP API
    // TODO: better error handling
    pub async fn put_items<T>(&self, base_name: String, items: Vec<T>) -> Result<()>
    where
        T: DeserializeOwned + Serialize {
        let client = Client::builder()
            .build::<_, Body>(HttpsConnector::new());
        let json = json!({
            "items": items
        });

        let request = Request::builder()
            .header("X-Api-Key", &self.project_key)
            .method(Method::PUT)
            .uri(format!("{API_BASE}/{}/{}/items", &self.project_id, base_name))
            .body(Body::from(serde_json::to_string(&json).unwrap()))
            .unwrap();
        client.request(request).await.unwrap();

        Ok(())
    }
}
