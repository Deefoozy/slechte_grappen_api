use crate::libs::db_connection::DatabaseConnection;
use crate::libs::model::Model;
use crate::models::file::File;
use crate::models::score_board::ScoreBoard;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Interface {
    pub id: i64,
    pub name: Option<String>,
    pub key: Option<String>,
    pub score_boards: Option<Vec<ScoreBoard>>,
    pub files: Option<Vec<File>>,
}

impl Interface {
    pub fn new(
        id: i64,
        name: Option<String>,
        key: Option<String>,
        score_boards: Option<Vec<ScoreBoard>>,
        files: Option<Vec<File>>,
    ) -> Self {
        Self {
            id,
            name,
            key,
            score_boards,
            files,
        }
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
        self.key = row.get(2);
    }

    pub async fn get_score_boards_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        let rows = Model::get_where(
            &db_conn,
            "interface_scoreboards",
            "interface_id",
            &self.id,
        )
            .await;

        let mut score_boards: Vec<ScoreBoard> = Vec::new();

        for row in rows {
            let temp_score_board = ScoreBoard::new_from_id(
                &db_conn,
                row.get(1),
            )
                .await;

            if let Ok(score_board) = temp_score_board {
                score_boards.push(score_board);
            }
        }

        self.score_boards = Option::from(score_boards);
    }

    pub async fn get_files(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        let rows = Model::get_where(
            &db_conn,
            "files",
            "interface_id",
            &self.id,
        )
            .await;

        let mut files: Vec<File> = Vec::new();

        for row in rows {
            files.push(
                File::new(
                    row.get(1),
                    row.get(2),
                    row.get(3),
                    None,
                )
            )
        }

        self.files = Option::from(files);
    }
}
