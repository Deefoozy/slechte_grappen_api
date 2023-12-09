use actix_web::{get, post, Responder, HttpResponse, web};
use crate::libs::db_connection::DatabaseConnection;
use crate::models::user::User;

#[get("/")]
pub async fn get_all() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/{id}")]
pub async fn get(id: web::Path<i64>) -> impl Responder {
    let db_conn = DatabaseConnection::new_from_env()
        .await;

    let user = User::new_from_id(
        &db_conn,
        *id,
        true,
    )
        .await;

    match user {
        Ok(user) => {
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
