use tokio_postgres::Error;
use crate::libs::db_connection::DatabaseConnection;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub async fn db_migrate() -> Result<(), Error> {
    let mut db_conn = DatabaseConnection::new_from_env()
        .await;

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
