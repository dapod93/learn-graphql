use crate::user::domain::entity::entity::User;

pub trait IUserRepository {
    fn get_by_id(&mut self, user_id: i32) -> Option<User>;
}

pub trait IUserUnitOfWork {
    fn user_repo(self) -> impl IUserRepository;
}
