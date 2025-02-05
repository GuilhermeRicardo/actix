use async_trait::async_trait;
use mongodb::{bson::oid::ObjectId, Client};
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait Crud: Sized + Send + Sync + for<'de> Deserialize<'de> + Serialize {
    async fn get(client: &Client, db_name: &str, id: &str) -> Option<Self>;
    async fn update(client: &Client, db_name: &str, id: &str, item: Self) -> bool;
    async fn delete(client: &Client, db_name: &str, id: &str) -> bool;
    async fn insert(client: &Client, db_name: &str, item: Self) -> Option<ObjectId>;
}