use crate::libs::db_connection::DatabaseConnection;
use crate::libs::model::Model;

pub struct User {
    pub id: i64,
    pub name: Option<String>,
}

impl User {
    pub fn new(
        id: i64,
        name: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
        }
    }

    pub async fn get_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        let result = Model::get_by_id(
            &db_conn,
            String::from("users"),
            &self.id,
        )
            .await;

        self.name = result.get(1);
    }
}
