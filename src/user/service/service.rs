use crate::user::domain::{
    entity::entity::User,
    interface::interface::{IUserRepository, IUserUnitOfWork},
};

pub fn get_users(uow: impl IUserUnitOfWork) -> Vec<Option<User>> {
    uow.user_repo().get_all()
}

pub fn get_user_by_id(uow: impl IUserUnitOfWork, user_id: i32) -> Option<User> {
    uow.user_repo().get_by_id(user_id)
}
