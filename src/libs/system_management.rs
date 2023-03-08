use tokio_postgres::{NoTls, Error};
use crate::check_env_key;
use crate::libs::db_connection;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub async fn db_migrate() -> Result<(), Error> {
    let mut db_conn = db_connection::DatabaseConnection::new(
        check_env_key("DB_HOST"),
        check_env_key("DB_USER"),
        check_env_key("DB_PASSWORD"),
        check_env_key("DB_NAME"),
    ).await;


    let migration_report = embedded::migrations::runner()
        .run_async(&mut db_conn.client)
        .await
        .unwrap();

    for migration in migration_report.applied_migrations() {
        println!(
            "Migration Applied -  Name: {}, Version: {}",
            migration.name(),
            migration.version()
        );
    }

    db_conn.close();

    Ok(())
}
