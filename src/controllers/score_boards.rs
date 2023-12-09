use actix_web::{get, post, Responder, HttpResponse, web};
use crate::libs::db_connection::DatabaseConnection;
use crate::models::score_board;

#[get("/")]
pub async fn get_all() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{id}")]
pub async fn get(id: web::Path<i64>) -> impl Responder {
    let db_conn = DatabaseConnection::new_from_env()
        .await;

    let mut score_board = score_board::ScoreBoard::new(
        *id,
        None,
        None,
        None,
        None,
        None
    );

    score_board.get_from_db(&db_conn).await;
    score_board.get_interfaces_from_db(&db_conn).await;
    score_board.get_users_from_db(&db_conn).await;

    HttpResponse::Ok().body(
        serde_json::to_string(&score_board).unwrap_or("{}".to_string())
    )
}

#[post("/")]
pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
