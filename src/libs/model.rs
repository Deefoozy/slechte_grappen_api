use tokio_postgres::Row;
use crate::libs::db_connection::DatabaseConnection;

pub struct Model {}

impl Model {
    pub async fn get_by_id(db_conn: &DatabaseConnection, table_name: &str, id: &i64) -> Row {
        db_conn.client.query_one(
            &format!("SELECT * FROM {} WHERE id = $1", &table_name),
            &[&id]
        )
            .await
            .unwrap()
    }

    pub async fn get_where(db_conn: &DatabaseConnection, table_name: &str, key: &str, id: &i64) -> Vec<Row> {
        db_conn.client.query(
            &format!("SELECT * FROM {} WHERE {} = $1", &table_name, key),
            &[&id]
        )
            .await
            .unwrap()
    }
}