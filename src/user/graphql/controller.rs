use std::sync::Arc;

use juniper::{FieldError, FieldResult, graphql_value};

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
        match get_user_by_id(UserUnitOfWork::new(self.db_pool.get()?), user_id) {
            None => Err(FieldError::new(
                "User not found",
                graphql_value!({"code": "NOT_FOUND"}),
            )),
            Some(u) => Ok(GetUserByIdResponse {
                id: u.id,
                first_name: u.first_name,
                last_name: u.last_name,
                email: u.email,
            }),
        }
    }
}
