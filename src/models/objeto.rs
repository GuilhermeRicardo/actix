use async_trait::async_trait;
use mongodb::{bson::{doc, oid::ObjectId}, Client};
use serde::{Deserialize, Serialize};
use crate::handlers::crud::Crud;

#[derive(Debug, Serialize, Deserialize)]
pub struct Objeto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub nome: String,
    pub valor: f64,
}

#[async_trait]
impl Crud for Objeto {
    async fn get(client: &Client, db_name: &str, id: &str) -> Option<Self> {
        let collection = client.database(db_name).collection::<Self>("objetos");
        let object_id = ObjectId::parse_str(id).ok()?;
        let filter = doc! { "_id": object_id };
        collection.find_one(filter, None).await.ok().flatten()
    }

    async fn update(client: &Client, db_name: &str, id: &str, item: Self) -> bool {
        let collection = client.database(db_name).collection::<Self>("objetos");
        let object_id = ObjectId::parse_str(id).ok();
        let filter = doc! { "_id": object_id };
        let update = doc! { "$set": { "nome": &item.nome, "valor": &item.valor } };
        let result = collection.update_one(filter, update, None).await;
        match result {
            Ok(update_result) => update_result.matched_count == 1,
            Err(_) => false,
        }
    }

    async fn delete(client: &Client, db_name: &str, id: &str) -> bool {
        let collection = client.database(db_name).collection::<Self>("objetos");
        let object_id = ObjectId::parse_str(id).ok();
        let filter = doc! { "_id": object_id };
        let result = collection.delete_one(filter, None).await;
        match result {
            Ok(delete_result) => delete_result.deleted_count == 1,
            Err(_) => false,
        }
    }

    async fn insert(client: &Client, db_name: &str, mut item: Self) -> Option<ObjectId> {
        let collection = client.database(db_name).collection::<Self>("objetos");
        let result = collection.insert_one(&item, None).await.ok()?;
        item.id = result.inserted_id.as_object_id();
        item.id
    }
}