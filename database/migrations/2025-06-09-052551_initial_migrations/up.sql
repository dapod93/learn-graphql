-- Your SQL goes here
CREATE TABLE IF EXISTS users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    first_name VARCHAR,
    last_name VARCHAR,
    email VARCHAR,
)