use crate::libs::db_connection::DatabaseConnection;
use crate::libs::model::Model;
use crate::models::score_board::ScoreBoard;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub score_boards: Vec<ScoreBoard>,
}

impl User {
    pub fn new(
        id: i64,
        name: String,
        score_boards: Vec<ScoreBoard>,
    ) -> Self {
        Self {
            id,
            name,
            score_boards
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
            "users",
            &id,
        )
            .await;

        Ok(
            User::new(
                id,
                row.get(1),
                Vec::new(),
            )
        )
    }

    pub async fn get_score_boards(db_conn: &DatabaseConnection, id: i64) -> Result<Vec<ScoreBoard>, ()> {
        let rows = Model::get_where(
            &db_conn,
            "user_scoreboards",
            "user_id",
            &id,
        )
            .await;

        let mut score_boards: Vec<ScoreBoard> = Vec::new();

        for row in rows {
            let temp_score_board: Result<ScoreBoard, ()> = ScoreBoard::new_from_id(
                db_conn,
                row.get(1),
            )
                .await;

            if let Ok(score_board) = temp_score_board {
                score_boards.push(score_board);
            }
        }

        return Ok(score_boards);
    }

    pub async fn load_relations(&mut self, db_conn: &DatabaseConnection) {
        self.get_score_boards_from_db(&db_conn).await;
    }

    pub async fn get_score_boards_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        self.score_boards = User::get_score_boards(db_conn, self.id)
            .await
            .unwrap();
    }
}
