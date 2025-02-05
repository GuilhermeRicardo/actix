use actix_web::{web, HttpResponse, Responder};
use mongodb::Client;
use crate::handlers::crud::Crud;

pub async fn get_item<T: Crud>(client: web::Data<Client>, db_name: web::Data<String>, id: web::Path<String>) -> impl Responder {
    match T::get(&client, &db_name, &id).await {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().body("Item não encontrado"),
    }
}

pub async fn update_item<T: Crud>(client: web::Data<Client>, db_name: web::Data<String>, id: web::Path<String>, item: web::Json<T>) -> impl Responder {
    if T::update(&client, &db_name, &id, item.into_inner()).await {
        match T::get(&client, &db_name, &id).await {
            Some(updated_item) => HttpResponse::Ok().json(serde_json::json!({
                "message": "Item atualizado com sucesso",
                "item": updated_item
            })),
            None => HttpResponse::InternalServerError().body("Erro ao buscar item atualizado"),
        }
    } else {
        HttpResponse::NotFound().body("Item não encontrado")
    }
}

pub async fn delete_item<T: Crud>(client: web::Data<Client>, db_name: web::Data<String>, id: web::Path<String>) -> impl Responder {
    if T::delete(&client, &db_name, &id).await {
        HttpResponse::Ok().body("Item deletado com sucesso")
    } else {
        HttpResponse::NotFound().body("Item não encontrado")
    }
}

pub async fn insert_item<T: Crud>(client: web::Data<Client>, db_name: web::Data<String>, item: web::Json<T>) -> impl Responder {
    match T::insert(&client, &db_name, item.into_inner()).await {
        Some(id) => match T::get(&client, &db_name, &id.to_hex()).await {
            Some(inserted_item) => HttpResponse::Ok().json(inserted_item),
            None => HttpResponse::InternalServerError().body("Erro ao buscar item inserido"),
        },
        None => HttpResponse::InternalServerError().body("Erro ao inserir item"),
    }
}