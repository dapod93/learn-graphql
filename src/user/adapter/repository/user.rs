use crate::common::orm::user::UserSQL;
use crate::database::schema::schema::users::dsl::*;
use crate::user::domain::{entity::entity::User, interface::interface::IUserRepository};

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{QueryDsl, prelude::*};

pub struct UserRepository {
    db_conn: PooledConnection<ConnectionManager<SqliteConnection>>,
}

impl IUserRepository for UserRepository {
    fn get_by_id(&mut self, user_id: i32) -> Option<User> {
        let query = QueryDsl::filter(users, id.eq(user_id))
            .first::<UserSQL>(&mut self.db_conn)
            .optional()
            .expect("error fetching user");
        self.model_to_entity(query)
    }
}

impl UserRepository {
    pub fn new(db_conn: PooledConnection<ConnectionManager<SqliteConnection>>) -> Self {
        UserRepository { db_conn }
    }

    fn model_to_entity(&self, model: Option<UserSQL>) -> Option<User> {
        match model {
            None => None,
            Some(m) => Some(User {
                id: m.id,
                first_name: m.first_name,
                last_name: m.last_name,
                email: m.email,
            }),
        }
    }
}
