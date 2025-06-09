use std::env;

use diesel::{Connection, SqliteConnection};

pub async fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("error connection to {}", database_url))
}
