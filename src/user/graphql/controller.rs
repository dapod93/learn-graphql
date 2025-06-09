use std::sync::Arc;

use juniper::FieldResult;

use crate::{
    database::connection::DbPool,
    user::{
        adapter::uow::internal::UserUnitOfWork, graphql::schema::response::GetUserByIdResponse,
        service::service::get_user_by_id,
    },
};

pub struct UserGraphQLController {
    db_pool: Arc<DbPool>,
}

impl UserGraphQLController {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        UserGraphQLController { db_pool }
    }

    pub fn get_user_by_id(&self, user_id: i32) -> FieldResult<GetUserByIdResponse> {
        let user = get_user_by_id(UserUnitOfWork::new(self.db_pool.get()?), user_id);
        Ok(GetUserByIdResponse {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        })
    }
}
