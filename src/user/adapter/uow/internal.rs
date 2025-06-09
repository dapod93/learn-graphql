use diesel::SqliteConnection;

use crate::{
    database::connection::DbPool,
    user::{
        adapter::repository::user::UserRepository,
        domain::interface::interface::{IUserRepository, IUserUnitOfWork},
    },
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
    pub fn new(db_pool: DbPool) -> Self {
        UserUnitOfWork {
            user_repo: UserRepository::new(db_pool),
        }
    }
}
