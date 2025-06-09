use crate::{common::orm::user::NewUserSQL, user::domain::entity::entity::User};

pub trait IUserRepository {
    fn create(&mut self, user: User) -> User;

    fn get_all(&mut self) -> Vec<Option<User>>;
    fn get_by_id(&mut self, user_id: i32) -> Option<User>;

    fn get_by_email(&mut self, email: String) -> Option<User>;
}

pub trait IUserUnitOfWork {
    fn user_repo(self) -> impl IUserRepository;
}
