use actix_web::http::header::q;
use sql_query_builder::Select;
use tokio_postgres::Row;
use crate::libs::db_connection::DatabaseConnection;
use crate::libs::query_builder;
use crate::libs::query_builder::WherePair;

pub struct Model {}

impl Model {
    pub async fn get_by_id(db_conn: &DatabaseConnection, table_name: &str, id: &i64) -> Row {
        db_conn.client.query_one(
            &query_builder::generate_single_clause_select(table_name, "id"),
            &[&id]
        )
            .await
            .unwrap()
    }

    pub async fn get_where(db_conn: &DatabaseConnection, table_name: &str, key: &str, id: &i64) -> Vec<Row> {
        db_conn.client.query(
            &query_builder::generate_single_clause_select(table_name, key),
            &[&id]
        )
            .await
            .unwrap()
    }
}