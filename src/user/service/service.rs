use crate::user::domain::{entity::entity::User, interface::interface::IUserUnitOfWork};

pub fn create_user(uow: &mut impl IUserUnitOfWork, user: User) -> User {
    uow.user_repo().create(user)
}

pub fn get_users(uow: &mut impl IUserUnitOfWork) -> Vec<Option<User>> {
    uow.user_repo().get_all()
}

pub fn get_user_by_id(uow: &mut impl IUserUnitOfWork, user_id: i32) -> Option<User> {
    uow.user_repo().get_by_id(user_id)
}

pub fn get_user_by_email(uow: &mut impl IUserUnitOfWork, email: String) -> Option<User> {
    uow.user_repo().get_by_email(email)
}
