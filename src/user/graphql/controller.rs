use std::sync::Arc;

use juniper::{FieldError, FieldResult, graphql_value};

use crate::{
    database::connection::DbPool,
    user::{
        adapter::uow::internal::UserUnitOfWork,
        graphql::schema::{request::CreateUserRequest, response::GetUserResponse},
        service::service::{get_user_by_email, get_user_by_id, get_users},
    },
};

pub struct UserGraphQLController {
    db_pool: Arc<DbPool>,
}

impl UserGraphQLController {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        UserGraphQLController { db_pool }
    }

    pub fn create_user(&self, req: CreateUserRequest) -> FieldResult<GetUserResponse> {
        let user = get_user_by_email(UserUnitOfWork::new(self.db_pool.get()?), req.email);
        match user {
            None => None,
            Some(u) => None,
        }
    }

    pub fn get_users(&self) -> FieldResult<Vec<Option<GetUserResponse>>> {
        Ok(get_users(UserUnitOfWork::new(self.db_pool.get()?))
            .into_iter()
            .map(|us| {
                us.map(|u| GetUserResponse {
                    id: u.id,
                    first_name: u.first_name,
                    last_name: u.last_name,
                    email: u.email,
                })
            })
            .collect())
    }

    pub fn get_user_by_id(&self, user_id: i32) -> FieldResult<GetUserResponse> {
        match get_user_by_id(UserUnitOfWork::new(self.db_pool.get()?), user_id) {
            None => Err(FieldError::new(
                "User not found",
                graphql_value!({"code": "NOT_FOUND"}),
            )),
            Some(u) => Ok(GetUserResponse {
                id: u.id,
                first_name: u.first_name,
                last_name: u.last_name,
                email: u.email,
            }),
        }
    }
}
