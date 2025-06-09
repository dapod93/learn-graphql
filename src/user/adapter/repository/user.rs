use crate::common::orm::user::UserSQL;
use crate::database::connection::DbPool;
use crate::database::schema::schema::users::dsl::*;
use crate::user::domain::{entity::entity::User, interface::interface::IUserRepository};

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{QueryDsl, prelude::*};

pub struct UserRepository {
    db_conn: PooledConnection<ConnectionManager<SqliteConnection>>,
}

impl IUserRepository for UserRepository {
    fn get_by_id(&mut self, user_id: i32) -> User {
        let query = QueryDsl::filter(users, id.eq(user_id))
            .first::<UserSQL>(&mut self.db_conn)
            .expect("error fetching user");
        self.model_to_entity(query)
    }
}

impl UserRepository {
    pub fn new(db_pool: DbPool) -> Self {
        let conn = db_pool.get().expect("Failed to get db conn from pool");
        UserRepository { db_conn: conn }
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
