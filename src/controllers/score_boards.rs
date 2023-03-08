use actix_web::{get, post, Responder, HttpResponse, web};

#[path="../models/score_board.rs"]
mod score_board;

#[get("/")]
pub async fn get_all() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{id}")]
pub async fn get(id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello world! {}", id))
}

#[post("/")]
pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
