use diesel::SqliteConnection;
use juniper::FieldResult;

use crate::{router::graphql::DbPool, user::{
    adapter::uow::internal::UserUnitOfWork, graphql::schema::response::GetUserByIdResponse,
    service::service::get_user_by_id,
}};

pub struct UserGraphQLController {
    db_pool: DbPool,
}

impl UserGraphQLController {
    pub fn new(db_pool: ) -> Self {
        UserGraphQLController { db_conn }
    }

    pub fn get_user_by_id(&self, user_id: i32) -> FieldResult<GetUserByIdResponse> {
        let db_conn

        let user = get_user_by_id(UserUnitOfWork::new(self.db_conn), user_id);
        Ok(GetUserByIdResponse {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        })
    }
}
