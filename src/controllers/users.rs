use std::num::NonZeroI64;

use actix_web::{get, post, Responder, HttpResponse, web};
use crate::libs::db_connection::DatabaseConnection;

use crate::repositories::UsersRepository;

#[get("/")]
pub async fn get_all() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{id}")]
pub async fn get(id: web::Path<NonZeroI64>) -> impl Responder {
    let db_conn = DatabaseConnection::new_from_env()
        .await;

    let repo = UsersRepository::new(db_conn);
    let user = repo.get_user_from_id(*id)
        .await;

    match user {
        Ok(mut user) => {
            let _unhandled_err = repo.load_related_scoreboards(&mut user).await;

            HttpResponse::Ok()
                .body(serde_json::to_string(&user).unwrap_or("{}".to_string()))
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
