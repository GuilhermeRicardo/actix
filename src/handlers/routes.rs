use actix_web::web;
use crate::handlers::db::{get_item, update_item, delete_item, insert_item};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/objeto/{id}", web::get().to(get_item::<crate::models::objeto::Objeto>))
            .route("/objeto/{id}", web::put().to(update_item::<crate::models::objeto::Objeto>))
            .route("/objeto/{id}", web::delete().to(delete_item::<crate::models::objeto::Objeto>))
            .route("/objeto", web::post().to(insert_item::<crate::models::objeto::Objeto>))
    );
}