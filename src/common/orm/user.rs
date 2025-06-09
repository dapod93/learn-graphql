use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
    sqlite::Sqlite,
};

use crate::database::schema::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name= users )]
#[diesel(check_for_backend(Sqlite))]
pub struct UserSQL {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name= users )]
#[diesel(check_for_backend(Sqlite))]
pub struct NewUserSQL {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
