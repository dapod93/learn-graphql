use diesel::{Selectable, prelude::Queryable};

use crate::database::schema::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name= users )]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserSQL {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
