use sea_query::Iden;

use crate::libs::db_connection::DatabaseConnection;
use crate::models::score_board::ScoreBoard;

#[derive(Iden)]
#[iden = "users"]
pub enum UserTableDefinition {
    Table,
    Id,
    Name,
}

pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub score_boards: Option<Vec<ScoreBoard>>,
}

impl User {
    pub fn new(
        id: i64,
        name: Option<String>,
        score_boards: Option<Vec<ScoreBoard>>,
    ) -> Self {
        Self {
            id,
            name,
            score_boards
        }
    }

    pub async fn get_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        // let row = Model::get_by_id(
        //     &db_conn,
        //     "users",
        //     &self.id,
        // )
        //     .await;
        //
        // self.name = row.get(1);
    }

    pub async fn get_score_boards_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        // let rows = Model::get_where_key(
        //     &db_conn,
        //     "user_scoreboards",
        //     "score_board_id",
        //     &self.id,
        // )
        //     .await;
        //
        // let mut score_boards: Vec<ScoreBoard> = Vec::new();
        //
        // for row in rows {
        //     score_boards.push(
        //         ScoreBoard::new(
        //             row.get(1),
        //             None,
        //             None,
        //             None,
        //             None,
        //             None
        //         )
        //     )
        // }
        //
        // self.score_boards = Option::from(score_boards);
    }
}
