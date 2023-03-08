use dotenv::dotenv;
use actix_web::{get, post, Responder, HttpServer, HttpResponse, App, web};
use tokio_postgres::{NoTls, Error};
use tokio;

mod models;
mod controllers;

fn check_env_key(key: &str) -> String {
    std::env::var(key).expect(
        format!("Key \"{}\" has an issue", key.to_string())
            .as_str()
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    check_env_key("DB_USER");
    check_env_key("DB_PASSWORD");
    check_env_key("DB_HOST");

    if check_env_key("RUN_MIGRATIONS") == "yes" {
        println!("Running migrations.");
        db_migrate().await.expect("Migrations failed.");
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
    println!("Get");

    // let res:Result<(),Error> = db_test().await;
    //
    // if res.is_err() {
    //     return HttpResponse::Unauthorized().body("Borked!");
    // }

    HttpResponse::Ok().body("Hello world!")
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

async fn db_migrate() -> Result<(), Error> {
    let user: String = check_env_key("DB_USER");
    let password: String = check_env_key("DB_PASSWORD");
    let host: String = check_env_key("DB_HOST");
    let database: String = check_env_key("DB_NAME");

    // Connect to the database.
    let (mut client, connection) =
        tokio_postgres::connect(
            format!("host={} user={} password={} dbname={}", host, user, password, database).as_str(),
            NoTls,
        )
            .await
            .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let migration_report = embedded::migrations::runner().run_async(&mut client).await.unwrap();

    for migration in migration_report.applied_migrations() {
        println!(
            "Migration Applied -  Name: {}, Version: {}",
            migration.name(),
            migration.version()
        );
    }

    Ok(())
}
