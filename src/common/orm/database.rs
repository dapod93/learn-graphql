use diesel::SqliteConnection;

use crate::database::connection::establish_connection;

pub struct DbConn {
    db_conn: SqliteConnection,
}

impl DbConn {
    pub fn new() -> Self {
        let db_conn = establish_connection();
        DbConn { db_conn }
    }
}
