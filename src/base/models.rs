use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Items<T> {
    pub items: Vec<T>,
}

impl<T> From<Vec<T>> for Items<T> {
    fn from(items: Vec<T>) -> Self {
        Self { items }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PutItemsResponse<T> {
    pub processed: Items<T>,
    pub failed: Items<T>,
}
