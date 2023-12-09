use actix_web::{get, post, Responder, HttpResponse, web};
use crate::libs::db_connection::DatabaseConnection;
use crate::models::score_board::ScoreBoard;

#[get("/")]
pub async fn get_all() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{id}")]
pub async fn get(id: web::Path<i64>) -> impl Responder {
    let db_conn = DatabaseConnection::new_from_env()
        .await;

    let score_board = ScoreBoard::new_from_id(
        &db_conn,
        *id,
    )
        .await;

    match score_board {
        Ok(mut score_board) => {
            score_board.load_relations(&db_conn).await;

            HttpResponse::Ok()
                .body(serde_json::to_string(&score_board).unwrap_or("{}".to_string()))
        },
        Err(_) => {
            HttpResponse::InternalServerError()
                .body("{}".to_string())
        }
    }
}

#[post("/")]
pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
