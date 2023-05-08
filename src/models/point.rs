use sea_query::Iden;

use crate::libs::db_connection::DatabaseConnection;

#[derive(Iden)]
#[iden = "points"]
pub enum PointTableDefinition {
    Table,
    Id,
    UserId,
    ScoreBoardId,
    Val,
    ScoreType,
}

pub struct Point {
    pub id: i64,
    pub user_id: Option<i64>,
    pub score_board_id: Option<i64>,
    pub val: Option<i32>,
    pub score_type: Option<String>,
}

impl Point {
    pub fn new(
        id: i64,
        user_id: Option<i64>,
        score_board_id: Option<i64>,
        val: Option<i32>,
        score_type: Option<String>,
    ) -> Self {
        Self {
            id,
            user_id,
            score_board_id,
            val,
            score_type,
        }
    }

    pub async fn get_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        // let row = Model::get_by_id(
        //     &db_conn,
        //     "points",
        //     &self.id,
        // )
        //     .await;
        //
        // self.user_id = row.get(1);
        // self.score_board_id = row.get(2);
        // self.val = row.get(3);
        // self.score_type = row.get(4);
    }
}
