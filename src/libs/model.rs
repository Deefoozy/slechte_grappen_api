use tokio_postgres::Row;
use crate::libs::db_connection::DatabaseConnection;

pub struct Model {}

impl Model {
    pub async fn get_by_id(
        db_conn: &DatabaseConnection,
        table_name: &str,
        id: &i64
    ) -> Row {
        Model::select_single(
            db_conn,
            table_name,
            &"id = $1",
            &[&id]
        )
            .await
    }

    pub async fn get_where(
        db_conn: &DatabaseConnection,
        table_name: &str,
        key: &str,
        id: &i64
    ) -> Vec<Row> {
        Model::select(
            db_conn,
            table_name,
            &format!("{} = $1", &key),
            &[&id]
        )
            .await
    }

    pub async fn select_single(
        db_conn: &DatabaseConnection,
        table_name: &str,
        where_statement: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)]
    ) -> Row {
        db_conn.client.query_one(
            &format!(
                "SELECT * FROM {} WHERE {}",
                &table_name,
                &where_statement
            ),
            params
        )
            .await
            .unwrap()
    }

    pub async fn select(
        db_conn: &DatabaseConnection,
        table_name: &str,
        where_statement: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)]
    ) -> Vec<Row> {
        db_conn.client.query(
            &format!(
                "SELECT * FROM {} WHERE {}",
                &table_name,
                &where_statement
            ),
            params
        )
            .await
            .unwrap()
    }
}