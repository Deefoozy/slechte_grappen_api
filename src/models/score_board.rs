use crate::libs::db_connection::DatabaseConnection;
use crate::libs::model::Model;

pub struct ScoreBoard {
    pub id: i64,
    pub name: Option<String>,
    pub point_increment: Option<i32>,
    pub penalty_increment: Option<i32>,
}

impl ScoreBoard {
    pub async fn new(
        id: i64,
        name: Option<String>,
        point_increment: Option<i32>,
        penalty_increment: Option<i32>,
    ) -> Self {
        Self {
            id,
            name,
            point_increment,
            penalty_increment,
        }
    }

    pub async fn get_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        let result = Model::get_by_id(
            &db_conn,
            String::from("score_boards"),
            &self.id,
        )
            .await;

        self.name = result.get(1);
        self.point_increment = result.get(2);
        self.penalty_increment = result.get(3);
    }
}
