use actix_web::{HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Bem-vindo ao Actix!")
}

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Olá, mundo!")
}