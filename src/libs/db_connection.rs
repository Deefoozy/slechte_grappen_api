use tokio_postgres::{NoTls, Error, Client};
use tokio;
use tokio::task::JoinHandle;

pub struct DatabaseConnection {
    handle: JoinHandle<()>,
    pub client: Client,
}

impl DatabaseConnection {
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
