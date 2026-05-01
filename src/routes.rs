use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, mime::JSON, post, web::{self, Json}};
use serde::{Deserialize, Serialize};

use crate::{input::{CreateOrderInput, DeleteOrder}, output::{CreateOrderResponse, DeleteOrderResponse, Depth}};


#[post("/order")]
pub async fn create_order(body: Json<CreateOrderInput>) -> impl Responder {
    println!("{:?}", body);

    // maintain orderbook logic
    

    HttpResponse::Ok().json(CreateOrderResponse{
        order_id: String::from("ads")
    })
}

#[delete("/order")]
pub async fn delete_order(Json(body): Json<DeleteOrder>) -> impl Responder{
    let order_id = body.order_id;

    HttpResponse::Ok().json(DeleteOrderResponse{
        filled_qty: 0,
        average_price: 100
    })
}

#[get("/depth")]
pub async fn get_depth() -> impl Responder{
    HttpResponse::Ok().json(Depth {
        bids: vec![],
        asks: vec![],
        last_update_id: String::from("adsa")
    })
}