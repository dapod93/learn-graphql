use crate::common::orm::user::UserSQL;
use crate::database::schema::schema::users::dsl::*;
use crate::user::domain::{entity::entity::User, interface::interface::IUserRepository};

use diesel::{QueryDsl, prelude::*};

pub struct UserRepository {
    conn: SqliteConnection,
}

impl IUserRepository for UserRepository {
    fn get_by_id(&mut self, user_id: i32) -> User {
        let query = QueryDsl::filter(users, id.eq(user_id))
            .first::<UserSQL>(&mut self.conn)
            .expect("error fetching user");
        self.model_to_entity(query)
    }
}

impl UserRepository {
    pub fn new(db_conn: SqliteConnection) -> Self {
        UserRepository { conn: db_conn }
    }

    fn model_to_entity(&self, model: UserSQL) -> User {
        User {
            id: model.id,
            first_name: model.first_name,
            last_name: model.last_name,
            email: model.email,
        }
    }
}
