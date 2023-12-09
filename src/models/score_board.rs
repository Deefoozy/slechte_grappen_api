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

    pub async fn get_interfaces(db_conn: &DatabaseConnection, id: i64) -> Result<Vec<Interface>, ()> {
        let rows = Model::get_where(
            &db_conn,
            "interface_scoreboards",
            "score_board_id",
            &id,
        )
            .await;

        let mut interfaces: Vec<Interface> = Vec::new();

        for row in rows {
            let temp_score_board: Interface = Interface::new(
                row.get(1),
                None,
                None,
                None,
                None,
            );

            interfaces.push(temp_score_board);
        }

        return Ok(interfaces);
    }

    pub async fn get_users(db_conn: &DatabaseConnection, id: i64) -> Result<Vec<User>, ()> {
        let rows = Model::get_where(
            &db_conn,
            "user_scoreboards",
            "score_board_id",
            &id,
        )
            .await;

        let mut users: Vec<User> = Vec::new();

        for row in rows {
            let temp_user: Result<User, ()> = User::new_from_id(
                db_conn,
                row.get(1),
            )
                .await;

            if let Ok(score_board) = temp_user {
                users.push(score_board);
            }
        }

        return Ok(users);
    }

    pub async fn load_relations(&mut self, db_conn: &DatabaseConnection) {
        self.get_interfaces_from_db(&db_conn).await;
        self.get_users_from_db(&db_conn).await;
    }

    pub async fn get_interfaces_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        self.interfaces = ScoreBoard::get_interfaces(db_conn, self.id)
            .await
            .unwrap();
    }

    pub async fn get_users_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        self.users = ScoreBoard::get_users(db_conn, self.id)
            .await
            .unwrap();
    }
}
