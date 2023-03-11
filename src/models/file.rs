use crate::libs::db_connection::DatabaseConnection;
use crate::libs::model::Model;

pub struct File {
    pub id: i64,
    pub interface_id: Option<i64>,
    pub file_type: Option<String>,
}

impl File {
    pub fn new(
        id: i64,
        interface_id: Option<i64>,
        file_type: Option<String>,
    ) -> Self {
        Self {
            id,
            interface_id,
            file_type,
        }
    }

    pub async fn get_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        let result = Model::get_by_id(
            &db_conn,
            String::from("files"),
            &self.id,
        )
            .await;

        self.interface_id = result.get(1);
        self.file_type = result.get(2);
    }
}
