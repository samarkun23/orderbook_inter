use std::sync::{Arc, Mutex};

use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, mime::JSON, post, web::{self, Data, Json}};
use serde::{Deserialize, Serialize};

use crate::{input::{CreateOrderInput, DeleteOrder}, orderbook::{self, Orderbook}, output::{CreateOrderResponse, DeleteOrderResponse, Depth}};


#[post("/order")]
pub async fn create_order(body: Json<CreateOrderInput>, orderbook: Data<Arc<Mutex<Orderbook>>>) -> impl Responder {
    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;
    
    // maintain orderbook logic
    let mut orderbook = orderbook.lock().unwrap(); // i was able to lock this thead over there . 
    orderbook.create_order(price,quantity, user_id, side);

    HttpResponse::Ok().json(CreateOrderResponse{
        order_id: String::from("ads")
    })
}

#[delete("/order")]
pub async fn delete_order(Json(body): Json<DeleteOrder>, orderbook: Data<Orderbook>) -> impl Responder{
    let order_id = body.order_id;

    HttpResponse::Ok().json(DeleteOrderResponse{
        filled_qty: 0,
        average_price: 100
    })
}

#[get("/depth")]
pub async fn get_depth(orderbook: Data<Orderbook>) -> impl Responder{
    HttpResponse::Ok().json(Depth {
        bids: vec![],
        asks: vec![],
        last_update_id: String::from("adsa")
    })
}