use actix_web::{get, post, Responder, HttpResponse, web};
use crate::libs::db_connection::DatabaseConnection;
use crate::libs::env_keys::check_env_key;
use crate::models::user::User;

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

    let mut user: User = User::new(
        id.into_inner(),
        None,
        None
    );

    user.get_from_db(&db_conn).await;
    user.get_score_boards_from_db(&db_conn).await;

    let group_name: &str =
        if user.score_boards.as_ref().unwrap().len() > 0 { user.score_boards.as_ref().unwrap()[0].name.as_ref().expect("No GroupName") }
        else { "No Groups" };

    HttpResponse::Ok().body(
        format!(
            "{} | {} | {}",
            user.id,
            user.name.as_ref().expect("No Name"),
            group_name
        )
    )
}

#[post("/")]
pub async fn create() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
