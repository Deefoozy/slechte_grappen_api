use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use tokio_postgres::SimpleQueryMessage;

use crate::libs::db_connection::DatabaseConnection;
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
                    .eq(self.id)
            )
            .to_string(PostgresQueryBuilder);

        let res = db_conn.client
            .simple_query(
                query.as_str()
            )
            .await
            .unwrap();

        for row in res {
            println!("{:?}", row);

            match row {
                SimpleQueryMessage::Row(row) => {
                    let id: i64 = row.get(0)
                        .and_then(|val| val.parse().ok())
                        .unwrap();

                    let name: String = row.get(1)
                        .map(|val| val.to_owned())
                        .unwrap();

                    let point_increment: i32 = row.get(2)
                        .and_then(|val| val.parse().ok())
                        .unwrap();

                    let penalty_increment: i32 = row.get(3)
                        .and_then(|val| val.parse().ok())
                        .unwrap();

                    println!(
                        "{:?}, {:?}, {:?}, {:?}",
                        id,
                        name,
                        point_increment,
                        penalty_increment
                    );
                },
                _ => (),
            }
        }

        // let row = Model::get_by_id(
        //     &db_conn,
        //     "score_boards",
        //     &self.id,
        // )
        //     .await;
        //
        // self.name = row.get(1);
        // self.point_increment = row.get(2);
        // self.penalty_increment = row.get(3);
    }

    pub async fn get_interfaces_from_db(&mut self, db_conn: &DatabaseConnection) {
        if self.id == 0 {
            return;
        };

        // let rows = Model::get_where_key(
        //     &db_conn,
        //     "interface_scoreboards",
        //     "score_board_id",
        //     &self.id,
        // )
        //     .await;
        //
        // let mut interfaces: Vec<Interface> = Vec::new();
        //
        // for row in rows {
        //     interfaces.push(
        //         Interface::new(
        //             row.get(1),
        //             None,
        //             None,
        //             None,
        //             None
        //         )
        //     )
        // }
        //
        // self.interfaces = Option::from(interfaces);
    }

    pub async fn get_users_from_db(&mut self, db_conn: &DatabaseConnection) {
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
        // let mut users: Vec<User> = Vec::new();
        //
        // for row in rows {
        //     users.push(
        //         User::new(
        //             row.get(1),
        //             None,
        //             None,
        //         )
        //     )
        // }
        //
        // self.users = Option::from(users);
    }
}
