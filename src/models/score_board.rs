use crate::libs::db_connection::DatabaseConnection;
use crate::libs::model::Model;
use crate::models::interface::Interface;
use crate::models::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ScoreBoard {
    pub id: i64,
    pub name: String,
    pub point_increment: i32,
    pub penalty_increment: Option<i32>,
    pub users: Vec<User>,
    pub interfaces: Vec<Interface>,
}

impl ScoreBoard {
    pub fn new(
        id: i64,
        name: String,
        point_increment: i32,
        penalty_increment: Option<i32>,
        users: Vec<User>,
        interfaces: Vec<Interface>,
    ) -> Self {
        Self {
            id,
            name,
            point_increment,
            penalty_increment,
            users,
            interfaces,
        }
    }

    pub async fn new_from_id(
        db_conn: &DatabaseConnection,
        id: i64,
    ) -> Result<Self, ()> {
        if id == 0 {
            return Err(());
        }

        let row = Model::get_by_id(
            &db_conn,
            "score_boards",
            &id,
        )
            .await;

        Ok(
            ScoreBoard::new(
                id,
                row.get(1),
                row.get(2),
                row.get(3),
                Vec::new(),
                Vec::new(),
            )
        )
    }

    pub async fn get_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        let row = Model::get_by_id(
            &db_conn,
            "score_boards",
            &self.id,
        )
            .await;

        self.name = row.get(1);
        self.point_increment = row.get(2);
        self.penalty_increment = row.get(3);
    }

    pub async fn load_relations(&mut self, db_conn: &DatabaseConnection) {
        self.get_interfaces_from_db(&db_conn).await;
        self.get_users_from_db(&db_conn).await;
    }

    pub async fn get_interfaces_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        let rows = Model::get_where(
            &db_conn,
            "interface_scoreboards",
            "score_board_id",
            &self.id,
        )
            .await;

        let mut interfaces: Vec<Interface> = Vec::new();

        for row in rows {
            let mut temp_interface:Interface = Interface::new(
                row.get(1),
                None,
                None,
                None,
                None
            );
            temp_interface.get_from_db(&db_conn).await;

            interfaces.push(temp_interface);
        }

        self.interfaces = interfaces;
    }

    pub async fn get_users_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        let rows = Model::get_where(
            &db_conn,
            "user_scoreboards",
            "score_board_id",
            &self.id,
        )
            .await;

        let mut users: Vec<User> = Vec::new();

        for row in rows {
            let temp_user_result = User::new_from_id(
                db_conn,
                row.get(1),
            )
                .await;

            if let Ok(user) = temp_user_result {
                users.push(user);
            }
        }

        self.users = users;
    }
}
