use tokio_postgres::Row;
use crate::libs::db_connection::DatabaseConnection;

pub struct Model {}

impl Model {
    pub async fn get_by_id(db_conn: &DatabaseConnection, table_name: String, id: &i64) -> Row {
        db_conn.client.query_one(
            "SELECT * FROM $1 WHERE id = $2",
            &[&table_name, &id.to_string()]
        )
            .await
            .unwrap()
    }
}