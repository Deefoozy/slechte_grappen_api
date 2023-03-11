use crate::libs::db_connection::DatabaseConnection;
use crate::libs::model::Model;

pub struct Interface {
    pub id: i64,
    pub name: Option<String>,
    pub key: Option<String>,
}

impl Interface {
    pub fn new(
        id: i64,
        name: Option<String>,
        key: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            key,
        }
    }

    pub async fn get_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        let result = Model::get_by_id(
            &db_conn,
            String::from("interfaces"),
            &self.id,
        )
            .await;

        self.name = result.get(1);
        self.key = result.get(2);
    }
}
