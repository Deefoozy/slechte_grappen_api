use tokio_postgres::{NoTls, Error};
use tokio;
use crate::check_env_key;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub async fn db_migrate() -> Result<(), Error> {
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