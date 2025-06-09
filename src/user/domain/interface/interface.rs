use crate::user::domain::entity::entity::User;

pub trait IUserRepository {
    fn get_by_id(id: i32) -> User;
}

pub trait IUserUnitOfWork {
    fn new();

    fn commit();
    fn rollback();
}
