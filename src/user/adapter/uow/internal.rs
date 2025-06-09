use diesel::SqliteConnection;

use crate::{
    database::connection::establish_connection,
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
    pub fn new() -> Self {
        UserUnitOfWork {
            user_repo: UserRepository::new(establish_connection()),
        }
    }
}
