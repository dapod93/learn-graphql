use std::env;

use diesel::{
    SqliteConnection,
    r2d2::{ConnectionManager, Pool},
};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(database_url))
        .expect("Failed to create db pool")
}
