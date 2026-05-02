use std::sync::{Arc, Mutex};

use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, web::{self, Json}};

use crate::{orderbook::Orderbook, routes::{create_order, delete_order, get_depth}};

pub mod routes;
pub mod input;
pub mod output;
pub mod orderbook; 

#[actix_web::main]
async fn main() {
    let orderbook = Arc::new(Mutex::new(Orderbook::new())); // this is single instance we need to give this all the endpoint 
    // we create arc of this orderbook it's give all the thread a orderbook clone kindoff and Mutex make sure that when anythead change anything it's lock the orderbook . 

    HttpServer::new(move || {
        App::new()
            .app_data(orderbook.clone())  // this is how you pass them all the endpoint
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

