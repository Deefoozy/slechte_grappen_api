use tokio_postgres::{Row, RowStream};
use futures_util::{pin_mut, TryStreamExt};

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

    pub async fn get_where_key(db_conn: &DatabaseConnection, table_name: &str, key: &str, id: &i64) -> Vec<Row> {
        Model::get_where(
            db_conn,
            table_name,
            &vec![WherePair::new(key, &id.to_string())]
        ).await
    }

    pub async fn get_where(db_conn: &DatabaseConnection, table_name: &str, pairs: &Vec<WherePair>) -> Vec<Row> {
        let query = query_builder::generate_multi_clause_select(table_name, &pairs);

        let values = WherePair::strip_values(&pairs);

        let res: RowStream = db_conn.client.query_raw(
            &query,
            values
        )
            .await
            .unwrap();

        pin_mut!(res);

        let mut rows: Vec<Row> = Vec::new();

        while let Some(row) = res.try_next().await.unwrap() {
            rows.push(row);
        }

        rows
    }
}