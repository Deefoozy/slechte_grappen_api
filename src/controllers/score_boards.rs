use actix_web::{get, post, Responder, HttpResponse, web};

use crate::libs::db_connection::DatabaseConnection;
use crate::libs::env_keys::check_env_key;
use crate::models::score_board;

#[get("/")]
pub async fn get_all() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{id}")]
pub async fn get(id: web::Path<i64>) -> impl Responder {
    let db_conn = DatabaseConnection::new(
        check_env_key("DB_HOST"),
        check_env_key("DB_USER"),
        check_env_key("DB_PASSWORD"),
        check_env_key("DB_NAME"),
    )
        .await;

    let mut score_board = score_board::ScoreBoard::new(
        id.into_inner(),
        None,
        None,
        None,
        None,
        None
    );

    score_board.get_from_db(&db_conn).await;
    // score_board.get_interfaces_from_db(&db_conn).await;

    db_conn.close();

    HttpResponse::Ok().body(
        format!(
            "Hello world! {} | {}",
            score_board.id,
            score_board.name.unwrap_or("".to_string())
        )
    )
}

#[post("/")]
pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
