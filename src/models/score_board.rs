use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use tokio_postgres::SimpleQueryMessage;

use crate::libs::db_connection::DatabaseConnection;
use crate::libs::model::Model;
use crate::models::interface::Interface;
use crate::models::user::User;

#[derive(Iden)]
#[iden = "score_boards"]
pub enum ScoreBoardTableDefinition {
    Table,
    Id,
    Name,
    PointIncrement,
    PenaltyIncrement,
}

pub enum ScoreBoardTableIndexes {
    Id = 0,
    Name = 1,
    PointIncrement = 2,
    PenaltyIncrement = 3,
}

pub struct ScoreBoard {
    pub id: i64,
    pub name: Option<String>,
    pub point_increment: Option<i32>,
    pub penalty_increment: Option<i32>,
    pub users: Option<Vec<User>>,
    pub interfaces: Option<Vec<Interface>>,
}

impl ScoreBoard {
    pub fn new(
        id: i64,
        name: Option<String>,
        point_increment: Option<i32>,
        penalty_increment: Option<i32>,
        users: Option<Vec<User>>,
        interfaces: Option<Vec<Interface>>,
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

    pub async fn get_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        let query = Query::select()
            .columns(
                vec![
                    ScoreBoardTableDefinition::Id,
                    ScoreBoardTableDefinition::Name,
                    ScoreBoardTableDefinition::PointIncrement,
                    ScoreBoardTableDefinition::PenaltyIncrement,
                ]
            )
            .from(ScoreBoardTableDefinition::Table)
            .and_where(
                Expr::col(ScoreBoardTableDefinition::Id)
                    .eq(self.id.clone())
            )
            .to_string(PostgresQueryBuilder);

        let res = db_conn.client
            .simple_query(
                query.as_str()
            )
            .await
            .unwrap();

        for row in res {
            match row {
                SimpleQueryMessage::Row(row) => {
                    self.name = Option::from(
                        row.get(ScoreBoardTableIndexes::Name as usize)
                            .unwrap()
                            .to_string()
                    );

                    let point_increment: i32 = row.get(ScoreBoardTableIndexes::PointIncrement as usize)
                        .unwrap()
                        .parse()
                        .unwrap();
                    self.point_increment = Option::from(point_increment);

                    let penalty_increment: i32 = row.get(ScoreBoardTableIndexes::PenaltyIncrement as usize)
                        .unwrap()
                        .parse()
                        .unwrap();
                    self.penalty_increment = Option::from(penalty_increment);
                },
                _ => (),
            }
        }
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

        self.interfaces = Option::from(interfaces);
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
            let mut temp_user = User::new(
                row.get(1),
                None,
                None,
            );
            temp_user.get_from_db(&db_conn).await;

            users.push(temp_user);
        }

        self.users = Option::from(users);
    }
}
