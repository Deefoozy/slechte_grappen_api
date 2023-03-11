mod models;
mod controllers;
mod libs;

use std::io::Result;
use dotenv::dotenv;
use actix_web::{get, Responder, HttpServer, HttpResponse, App, web};
use crate::libs::env_keys::check_env_key;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    if check_env_key("RUN_MIGRATIONS") == "yes" {
        println!("Running migrations.");

        libs::system_management::db_migrate()
            .await
            .expect("Migrations failed.");
    } else {
        println!("Skipping migrations.");
    };

    println!("Starting server");

    HttpServer::new(|| {
        App::new()
            // .service(test)
            .service(
                web::scope("/score_boards")
                    .service(controllers::score_boards::get_all)
                    .service(controllers::score_boards::get)
                    .service(controllers::score_boards::create)
            )
    })
        .bind(format!("0.0.0.0:{}", check_env_key("APP_PORT")))?
        .run()
        .await
}

#[get("/")]
async fn test() -> impl Responder {
    println!("TEST CALLED");

    HttpResponse::Ok().body("Hello world!")
}
