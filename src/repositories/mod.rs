use std::num::NonZeroI64;

use crate::{
    libs::{db_connection::DatabaseConnection, model::Model}, 
    models::user::User
};

pub struct UsersRepository {
    db_conn: DatabaseConnection,
    table_name: Box<str>
}

impl UsersRepository {

    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self {
            db_conn,
            table_name: "users".into()
        }
    }

    pub async fn get_user_from_id(&self, user_id: NonZeroI64) -> Result<User, ()> {

        let row = Model::get_by_id(
            &self.db_conn,
            &self.table_name,
            &user_id.get(),
        )
            .await;

        Ok(
            User::new(
                user_id.get(),
                row.get(1),
                Vec::new(),
            )
        )

    }

    pub async fn load_related_scoreboards(&self, user: &mut User) -> Result<(), ()> {
        if user.id <= 0 {
            return Err(());
        };

        User::get_score_boards(&self.db_conn, user.id)
            .await
            .map(|scoreboards| user.score_boards = scoreboards)
    }

}