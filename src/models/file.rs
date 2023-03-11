use crate::libs::db_connection::DatabaseConnection;
use crate::libs::model::Model;
use crate::models::interface::Interface;

pub struct File {
    pub id: i64,
    pub interface_id: Option<i64>,
    pub file_type: Option<String>,
    pub interface: Option<Interface>,
}

impl File {
    pub fn new(
        id: i64,
        interface_id: Option<i64>,
        file_type: Option<String>,
        interface: Option<Interface>,
    ) -> Self {
        Self {
            id,
            interface_id,
            file_type,
            interface
        }
    }

    pub async fn get_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        let row = Model::get_by_id(
            &db_conn,
            "files",
            &self.id,
        )
            .await;

        self.interface_id = row.get(1);
        self.file_type = row.get(2);
    }

    pub async fn get_interface(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        let row = Model::get_by_id(
            &db_conn,
            "interface",
            &self.id,
        )
            .await;

        self.interface = Option::from(
            Interface::new(
                row.get(1),
                row.get(2),
                row.get(3),
                None,
                None,
            )
        );
    }
}
