use std::sync::Arc;

use juniper::{FieldError, FieldResult, graphql_value};

use crate::{
    database::connection::DbPool,
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
        let mut uow = UserUnitOfWork::new(self.db_pool.get()?);
        match get_user_by_email(&mut uow, req.email.clone()) {
            None => {
                let user = create_user(
                    &mut uow,
                    User {
                        id: None,
                        first_name: req.first_name,
                        last_name: req.last_name,
                        email: req.email,
                    },
                );
                Ok(GetUserResponse {
                    id: user.id.unwrap(),
                    first_name: user.first_name,
                    last_name: user.last_name,
                    email: user.email,
                })
            }
            Some(_) => Err(FieldError::new(
                "User alread exists",
                graphql_value!({"code": "CONFLICT"}),
            )),
        }
    }

    pub fn get_users(&self) -> FieldResult<Vec<Option<GetUserResponse>>> {
        Ok(get_users(&mut UserUnitOfWork::new(self.db_pool.get()?))
            .into_iter()
            .map(|us| {
                us.map(|u| GetUserResponse {
                    id: u.id.unwrap(),
                    first_name: u.first_name,
                    last_name: u.last_name,
                    email: u.email,
                })
            })
            .collect())
    }

    pub fn get_user_by_id(&self, user_id: i32) -> FieldResult<GetUserResponse> {
        match get_user_by_id(&mut UserUnitOfWork::new(self.db_pool.get()?), user_id) {
            None => Err(FieldError::new(
                "User not found",
                graphql_value!({"code": "NOT_FOUND"}),
            )),
            Some(u) => Ok(GetUserResponse {
                id: u.id.unwrap(),
                first_name: u.first_name,
                last_name: u.last_name,
                email: u.email,
            }),
        }
    }
}
