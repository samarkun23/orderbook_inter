use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, web::{self, Json}};
use serde::{Deserialize, Serialize};

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


#[derive(Deserialize, Debug)]
enum Side {
    Buy,
    Sell
}


#[derive(Serialize , Deserialize, Debug)]
struct CreateOrderResponse{
    order_id: String
}

#[derive(Deserialize, Debug)]
struct CreateOrderInput {
    price: u32,
    quantity: u32,
    userId : u32,
    side: Side
}

#[post("/order")]
async fn create_order(body: Json<CreateOrderInput>) -> impl Responder {
    println!("{:?}", body);

    // maintain orderbook logic
    

    HttpResponse::Ok().json(CreateOrderResponse{
        order_id: String::from("ads")
    })
}

#[delete("/order")]
async fn delete_order() -> impl Responder{
    HttpResponse::Ok().body("Delete order ep")
}

#[get("/depth")]
async fn get_depth() -> impl Responder{
    HttpResponse::Ok().body("Get depth ep")
}