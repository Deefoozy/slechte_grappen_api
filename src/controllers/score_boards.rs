use actix_web::{get, post, Responder, HttpResponse, web};
use crate::models::score_board;

#[get("/")]
pub async fn get_all() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{id}")]
pub async fn get(id: web::Path<i64>) -> impl Responder {
    let score_board = score_board::ScoreBoard::new(id.into_inner())
        .await;

    score_board.close();

    HttpResponse::Ok().body(format!("Hello world! {} | {}", score_board.id, score_board.name))
}

#[post("/")]
pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
