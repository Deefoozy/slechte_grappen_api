use tokio_postgres::{NoTls, Client};
use tokio;
use tokio::task::JoinHandle;
use crate::libs::env_keys::check_env_key;

pub struct DatabaseConnection {
    handle: JoinHandle<()>,
    pub client: Client,
}

impl DatabaseConnection {
    pub async fn new_from_env() -> Self {
        DatabaseConnection::new(
            check_env_key("DB_HOST"),
            check_env_key("DB_USER"),
            check_env_key("DB_PASSWORD"),
            check_env_key("DB_NAME"),
        )
            .await
    }

    pub async fn new(host: String, user: String, password: String, database: String) -> Self {
        let (client, connection) =
            tokio_postgres::connect(
                format!(
                    "host={} user={} password={} dbname={}",
                    host,
                    user,
                    password,
                    database,
                )
                    .as_str(),
                NoTls,
            )
                .await
                .unwrap();

        let handle = tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Self {
            handle,
            client,
        }
    }

    pub fn close(&self) {
        self.handle.abort();
    }
}
