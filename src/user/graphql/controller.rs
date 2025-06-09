use diesel::SqliteConnection;
use juniper::FieldResult;

use crate::user::{
    adapter::uow::internal::UserUnitOfWork, graphql::schema::response::GetUserByIdResponse,
    service::service::get_user_by_id,
};

pub struct UserGraphQLController {
    db_conn: SqliteConnection,
}

impl UserGraphQLController {
    pub fn get_user_by_id(self, user_id: i32) -> FieldResult<GetUserByIdResponse> {
        let user = get_user_by_id(UserUnitOfWork::new(self.db_conn), user_id);
        Ok(GetUserByIdResponse {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        })
    }
}
