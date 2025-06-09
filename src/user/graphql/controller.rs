use std::sync::Arc;

use juniper::{FieldError, FieldResult, graphql_value};

use crate::{
    common::orm::user::NewUserSQL,
    database::{connection::DbPool, schema::schema::users::last_name},
    user::{
        adapter::uow::internal::UserUnitOfWork,
        domain::entity::entity::User,
        graphql::schema::{request::CreateUserRequest, response::GetUserResponse},
        service::service::{create_user, get_user_by_email, get_user_by_id, get_users},
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
        let uow = UserUnitOfWork::new(self.db_pool.get()?);
        let user = get_user_by_email(uow, req.email);
        match user {
            None => create_user(
                uow,
                User {
                    id: (),
                    first_name: req.first_name,
                    last_name: req.last_name,
                    email: req.email,
                },
            )
            Ok(GetUserResponse { id, first_name, last_name, email })
            ,
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
