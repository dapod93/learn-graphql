use diesel::{
    SqliteConnection,
    r2d2::{ConnectionManager, PooledConnection},
};

use crate::user::{
    adapter::repository::user::UserRepository,
    domain::interface::interface::{IUserRepository, IUserUnitOfWork},
};

pub struct UserUnitOfWork {
    user_repo: UserRepository,
}

impl IUserUnitOfWork for UserUnitOfWork {
    fn user_repo(self) -> impl IUserRepository {
        self.user_repo
    }
}

impl UserUnitOfWork {
    pub fn new(db_conn: PooledConnection<ConnectionManager<SqliteConnection>>) -> Self {
        UserUnitOfWork {
            user_repo: UserRepository::new(db_conn),
        }
    }
}
