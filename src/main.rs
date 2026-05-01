use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, web::{self, Json}};

use crate::routes::{create_order, delete_order, get_depth};

pub mod routes;
pub mod input;
pub mod output;

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("Hello world".to_string()) }),
            )
            .service(create_order)
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .await
    .unwrap()
}

