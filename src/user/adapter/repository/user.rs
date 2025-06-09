use crate::{
    common::orm::user::User, database::schema::schema::users,
    user::domain::interface::interface::IUserRepository,
};

pub struct UserRepository {}

impl IUserRepository for UserRepository {
    fn get_by_id(id: i32) -> crate::user::domain::entity::entity::User {
        diesel::
    }
}
