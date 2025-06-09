use crate::user::domain::entity::entity::User;

trait IUserRepository {
    fn get_by_id(id: i32) -> User;
}

trait IUserUnitOfWork {
    fn new();

    fn commit();
    fn rollback();
}
