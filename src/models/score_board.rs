use crate::libs::db_connection::DatabaseConnection;
use crate::check_env_key;

pub struct ScoreBoard {
    db_conn: DatabaseConnection,
    pub id: i64,
    pub name: String,
    pub point_increment: Option<i32>,
    pub penalty_increment: Option<i32>
}

impl ScoreBoard {
    pub async fn new(id: i64) -> Self {
        let db_conn = DatabaseConnection::new(
            check_env_key("DB_HOST"),
            check_env_key("DB_USER"),
            check_env_key("DB_PASSWORD"),
            check_env_key("DB_NAME"),
        )
            .await;

        let result = db_conn.client.query_one(
            "SELECT * FROM score_boards WHERE id = $1",
            &[&id]
        )
            .await
            .unwrap();

        let name: String = result.get(1);
        let point_increment: Option<i32> = result.get(2);
        let penalty_increment: Option<i32> = result.get(3);

        Self {
            db_conn,
            id,
            name,
            point_increment,
            penalty_increment,
        }
    }

    pub fn close(&self) {
        self.db_conn.close();
    }
}
