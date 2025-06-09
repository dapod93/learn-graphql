use crate::common::orm::user::UserSQL;
use crate::database::schema::schema::users::dsl::*;
use crate::user::domain::{entity::entity::User, interface::interface::IUserRepository};

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{QueryDsl, prelude::*};

pub struct UserRepository {
    db_conn: PooledConnection<ConnectionManager<SqliteConnection>>,
}

impl IUserRepository for UserRepository {
    fn get_all(&mut self) -> Vec<Option<User>> {
        users
            .load::<UserSQL>(&mut self.db_conn)
            .unwrap_or_else(|_| vec![])
            .into_iter()
            .map(|model| self.model_to_entity(Some(model)))
            .collect()
    }

    fn get_by_id(&mut self, user_id: i32) -> Option<User> {
        QueryDsl::filter(users, id.eq(user_id))
            .first::<UserSQL>(&mut self.db_conn)
            .optional()
            .map(|model| self.model_to_entity(model))
            .expect("error fetching user")
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
