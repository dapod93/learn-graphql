use crate::{
    database::schema::schema::users::dsl::*,
    user::domain::{entity::entity::User, interface::interface::IUserRepository},
};

use diesel::{prelude::*, query_dsl::methods::FilterDsl};

pub struct UserRepository {
    conn: SqliteConnection,
}

impl IUserRepository for UserRepository {
    fn get_by_id(&mut self, user_id: i32) -> crate::user::domain::entity::entity::User {
        users
            .filter(id.eq(user_id))
            .first::<User>(&mut self.conn)
            .expect("error fetching user")
    }
}
